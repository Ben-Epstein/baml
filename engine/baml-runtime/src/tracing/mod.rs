mod api_wrapper;
#[cfg(not(feature = "wasm"))]
mod threaded_tracer;
#[cfg(feature = "wasm")]
mod wasm_tracer;

use anyhow::Result;
use indexmap::IndexMap;
use std::collections::HashMap;

use uuid::Uuid;

use crate::{FunctionResult, RuntimeContext};

use self::api_wrapper::{
    core_types::{EventChain, IOValue, LogSchema, LogSchemaContext, TypeSchema, IO},
    APIWrapper,
};
#[cfg(not(feature = "wasm"))]
use self::threaded_tracer::ThreadedTracer;

#[cfg(feature = "wasm")]
use self::wasm_tracer::NonThreadedTracer;

#[cfg(not(feature = "wasm"))]
type TracerImpl = ThreadedTracer;
#[cfg(feature = "wasm")]
type TracerImpl = NonThreadedTracer;

pub struct TracingSpan {
    span_id: Uuid,
    function_name: String,
    params: IndexMap<String, serde_json::Value>,
    parent_ids: Option<Vec<(String, Uuid)>>,
    start_time: std::time::Instant,
    ctx: RuntimeContext,
}

pub struct BamlTracer {
    options: APIWrapper,
    enabled: bool,
    tracer: Option<TracerImpl>,
}

impl BamlTracer {
    pub fn new(options: Option<APIWrapper>, ctx: &RuntimeContext) -> Self {
        #[cfg(not(feature = "wasm"))]
        let options = options.unwrap_or_default();
        #[cfg(feature = "wasm")]
        let options = options.unwrap_or_else(|| ctx.into());

        BamlTracer {
            tracer: if options.enabled() {
                Some(TracerImpl::new(
                    &options,
                    if options.stage() == "test" { 1 } else { 20 },
                ))
            } else {
                None
            },
            enabled: options.enabled(),
            options,
        }
    }

    pub(crate) fn flush(&self) -> Result<()> {
        if let Some(tracer) = &self.tracer {
            tracer.flush()
        } else {
            Ok(())
        }
    }

    pub(crate) fn start_span(
        &self,
        function_name: &str,
        ctx: &RuntimeContext,
        params: &IndexMap<String, serde_json::Value>,
        parent: Option<&TracingSpan>,
    ) -> Option<TracingSpan> {
        if !self.enabled {
            return None;
        }
        Some(TracingSpan {
            span_id: Uuid::new_v4(),
            function_name: function_name.to_string(),
            params: params.clone(),
            parent_ids: parent.map(|p| vec![(p.function_name.clone(), p.span_id)]),
            start_time: std::time::Instant::now(),
            ctx: ctx.clone(),
        })
    }

    pub(crate) async fn finish_span(
        &self,
        span: TracingSpan,
        response: Option<serde_json::Value>,
    ) -> Result<()> {
        if let Some(tracer) = &self.tracer {
            tracer.submit((&self.options, span, response).into()).await
        } else {
            Ok(())
        }
    }

    pub(crate) async fn finish_baml_span(
        &self,
        span: TracingSpan,
        response: &Result<FunctionResult>,
    ) -> Result<()> {
        if let Some(tracer) = &self.tracer {
            tracer.submit((&self.options, span, response).into()).await
        } else {
            Ok(())
        }
    }
}

impl From<(&APIWrapper, &TracingSpan)> for LogSchemaContext {
    fn from((api, span): (&APIWrapper, &TracingSpan)) -> Self {
        let parents = &span.parent_ids.as_ref();
        LogSchemaContext {
            hostname: api.host_name().to_string(),
            stage: Some(api.stage().to_string()),
            latency_ms: span.start_time.elapsed().as_millis() as i128,
            process_id: api.session_id().to_string(),
            tags: HashMap::new(),
            event_chain: parents
                .map(|p| {
                    p.iter()
                        .map(|(name, _)| EventChain {
                            function_name: name.clone(),
                            variant_name: None,
                        })
                        .collect()
                })
                .unwrap_or_default(),
            start_time: "o".into(),
        }
    }
}

impl From<&IndexMap<String, serde_json::Value>> for IOValue {
    fn from(items: &IndexMap<String, serde_json::Value>) -> Self {
        IOValue {
            r#type: TypeSchema {
                name: api_wrapper::core_types::TypeSchemaName::Multi,
                fields: items
                    .iter()
                    // TODO: @hellovai do better types
                    .map(|(k, v)| (k.clone(), "unknown".into()))
                    .collect::<IndexMap<_, _>>(),
            },
            value: api_wrapper::core_types::ValueType::List(
                items
                    .iter()
                    .map(|(_, v)| serde_json::to_string(v).unwrap())
                    .collect::<Vec<_>>(),
            ),
            r#override: None,
        }
    }
}

impl From<&serde_json::Value> for IOValue {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Object(obj) => {
                let fields = obj
                    .iter()
                    .map(|(k, v)| (k.clone(), "unknown".into()))
                    .collect::<IndexMap<_, _>>();
                IOValue {
                    r#type: TypeSchema {
                        name: api_wrapper::core_types::TypeSchemaName::Multi,
                        fields,
                    },
                    value: api_wrapper::core_types::ValueType::List(
                        obj.iter()
                            .map(|(_, v)| serde_json::to_string(v).unwrap())
                            .collect::<Vec<_>>(),
                    ),
                    r#override: None,
                }
            }
            _ => IOValue {
                r#type: TypeSchema {
                    name: api_wrapper::core_types::TypeSchemaName::Single,
                    fields: [("value".into(), "unknown".into())].into(),
                },
                value: api_wrapper::core_types::ValueType::String(value.to_string()),
                r#override: None,
            },
        }
    }
}

impl From<(&APIWrapper, TracingSpan, Option<serde_json::Value>)> for LogSchema {
    fn from((api, span, result): (&APIWrapper, TracingSpan, Option<serde_json::Value>)) -> Self {
        let parent_ids = &span.parent_ids.as_ref();
        LogSchema {
            project_id: api.project_id().map(|s| s.to_string()),
            event_type: api_wrapper::core_types::EventType::FuncCode,
            root_event_id: parent_ids
                .and_then(|p| p.first().map(|(_, id)| *id))
                .unwrap_or(span.span_id)
                .to_string(),
            event_id: span.span_id.to_string(),
            parent_event_id: parent_ids.and_then(|p| p.last().map(|(_, id)| id.to_string())),
            context: (api, &span).into(),
            io: IO {
                input: Some((&span.params).into()),
                output: result.as_ref().map(|r| r.into()),
            },
            error: None,
            metadata: None,
        }
    }
}

fn error_from_result(result: &Result<FunctionResult>) -> Option<api_wrapper::core_types::Error> {
    match result {
        Ok(r) if r.parsed.is_some() => None,
        Ok(r) => Some(api_wrapper::core_types::Error {
            code: -2,
            message: r
                .parsed
                .as_ref()
                .and_then(|r| r.as_ref().err().map(|e| e.to_string()))
                .or_else(|| r.llm_response.content().err().map(|e| e.to_string()))
                .unwrap_or_else(|| "Unknown error".to_string()),
            traceback: None,
            r#override: None,
        }),
        Err(e) => Some(api_wrapper::core_types::Error {
            code: -2,
            message: e.to_string(),
            traceback: None,
            r#override: None,
        }),
    }
}

impl From<(&APIWrapper, TracingSpan, &Result<FunctionResult>)> for LogSchema {
    fn from((api, span, result): (&APIWrapper, TracingSpan, &Result<FunctionResult>)) -> Self {
        LogSchema {
            project_id: api.project_id().map(|s| s.to_string()),
            event_type: api_wrapper::core_types::EventType::FuncCode,
            root_event_id: span.span_id.to_string(),
            event_id: span.span_id.to_string(),
            parent_event_id: None,
            context: (api, &span).into(),
            io: IO {
                input: Some((&span.params).into()),
                output: result
                    .as_ref()
                    .ok()
                    .and_then(|r| r.parsed.as_ref())
                    .and_then(|r| r.as_ref().ok())
                    .map(|(r, _)| r.into()),
            },
            error: error_from_result(result),
            metadata: None,
        }
    }
}