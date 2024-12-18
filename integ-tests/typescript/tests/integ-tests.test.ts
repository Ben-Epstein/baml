import assert from 'assert'
import { Image, ClientRegistry, BamlValidationError } from '@boundaryml/baml'
import TypeBuilder from '../baml_client/type_builder'
import { scheduler } from 'node:timers/promises'
import { image_b64, audio_b64 } from './base64_test_data'
import { Audio } from '@boundaryml/baml'
import {
  b,
  NamedArgsSingleEnumList,
  flush,
  traceAsync,
  traceSync,
  setTags,
  TestClassNested,
  onLogEvent,
  AliasedEnum,
  MapKey,
} from '../baml_client'
import { RecursivePartialNull } from '../baml_client/async_client'
import { b as b_sync } from '../baml_client/sync_client'
import { config } from 'dotenv'
import { BamlLogEvent, BamlRuntime } from '@boundaryml/baml/native'
import { AsyncLocalStorage } from 'async_hooks'
import { DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME, resetBamlEnvVars } from '../baml_client/globals'
import exp from 'constants'
config()

describe('Integ tests', () => {

  

  describe('should work for all inputs', () => {
    it('single bool', async () => {
      const res = await b.TestFnNamedArgsSingleBool(true)
      expect(res).toEqual('true')
    })

    it('single string list', async () => {
      const res = await b.TestFnNamedArgsSingleStringList(['a', 'b', 'c'])
      expect(res).toContain('a')
      expect(res).toContain('b')
      expect(res).toContain('c')
    })

    it('return literal union', async () => {
      const res = await b.LiteralUnionsTest('a')
      expect(res == 1 || res == true || res == 'string output').toBeTruthy()
    })

    it('single class', async () => {
      console.log('calling with class')
      const res = await b.TestFnNamedArgsSingleClass({
        key: 'key',
        key_two: true,
        key_three: 52,
      })
      console.log('got response', res)
      expect(res).toContain('52')
    })

    it('multiple classes', async () => {
      const res = await b.TestMulticlassNamedArgs(
        {
          key: 'key',
          key_two: true,
          key_three: 52,
        },
        {
          key: 'key',
          key_two: true,
          key_three: 64,
        },
      )
      expect(res).toContain('52')
      expect(res).toContain('64')
    })

    it('single enum list', async () => {
      const res = await b.TestFnNamedArgsSingleEnumList([NamedArgsSingleEnumList.TWO])
      expect(res).toContain('TWO')
    })

    it('single float', async () => {
      const res = await b.TestFnNamedArgsSingleFloat(3.12)
      expect(res).toContain('3.12')
    })

    it('single int', async () => {
      const res = await b.TestFnNamedArgsSingleInt(3566)
      expect(res).toContain('3566')
    })

    it('single literal int', async () => {
      const res = await b.TestNamedArgsLiteralInt(1)
      expect(res).toContain('1')
    })

    it('single literal bool', async () => {
      const res = await b.TestNamedArgsLiteralBool(true)
      expect(res).toContain('true')
    })

    it('single literal string', async () => {
      const res = await b.TestNamedArgsLiteralString('My String')
      expect(res).toContain('My String')
    })

    it('single class with literal prop', async () => {
      const res = await b.FnLiteralClassInputOutput({ prop: 'hello' })
      expect(res).toEqual({ prop: 'hello' })
    })

    it('single class with literal union prop', async () => {
      const res = await b.FnLiteralUnionClassInputOutput({ prop: 'one' })
      expect(res).toEqual({ prop: 'one' })
    })

    it('single optional string', async () => {
      // TODO fix the fact it's required.
      const res = await b.FnNamedArgsSingleStringOptional()
    })

    it('single map string to string', async () => {
      const res = await b.TestFnNamedArgsSingleMapStringToString({ lorem: 'ipsum', dolor: 'sit' })
      expect(res).toHaveProperty('lorem', 'ipsum')
    })

    it('single map string to class', async () => {
      const res = await b.TestFnNamedArgsSingleMapStringToClass({ lorem: { word: 'ipsum' }, dolor: { word: 'sit' } })
      expect(res).toHaveProperty('lorem', { word: 'ipsum' })
    })

    it('single map string to map', async () => {
      const res = await b.TestFnNamedArgsSingleMapStringToMap({ lorem: { word: 'ipsum' }, dolor: { word: 'sit' } })
      expect(res).toHaveProperty('lorem', { word: 'ipsum' })
    })

    it('enum key in map', async () => {
      const res = await b.InOutEnumMapKey({ [MapKey.A]: 'A' }, { [MapKey.B]: 'B' })
      expect(res).toHaveProperty(MapKey.A, 'A')
      expect(res).toHaveProperty(MapKey.B, 'B')
    })

    it('literal string union key in map', async () => {
      const res = await b.InOutLiteralStringUnionMapKey({ one: '1' }, { two: '2' })
      expect(res).toHaveProperty('one', '1')
      expect(res).toHaveProperty('two', '2')
    })

    it('single literal string key in map', async () => {
      const res = await b.InOutSingleLiteralStringMapKey({ key: '1' })
      expect(res).toHaveProperty('key', '1')
    })

    it('primitive union alias', async () => {
      const res = await b.PrimitiveAlias('test')
      expect(res).toEqual('test')
    })

    it('map alias', async () => {
      const res = await b.MapAlias({ A: ['B', 'C'], B: [], C: [] })
      expect(res).toEqual({ A: ['B', 'C'], B: [], C: [] })
    })

    it('alias union', async () => {
      let res = await b.NestedAlias('test')
      expect(res).toEqual('test')

      res = await b.NestedAlias({ A: ['B', 'C'], B: [], C: [] })
      expect(res).toEqual({ A: ['B', 'C'], B: [], C: [] })
    })

    it('alias pointing to recursive class', async () => {
      const res = await b.AliasThatPointsToRecursiveType({ value: 1, next: null })
      expect(res).toEqual({ value: 1, next: null })
    })

    it('class pointing to alias that points to recursive class', async () => {
      const res = await b.ClassThatPointsToRecursiveClassThroughAlias({ list: { value: 1, next: null } })
      expect(res).toEqual({ list: { value: 1, next: null } })
    })

    it('recursive class with alias indirection', async () => {
      const res = await b.RecursiveClassWithAliasIndirection({ value: 1, next: { value: 2, next: null } })
      expect(res).toEqual({ value: 1, next: { value: 2, next: null } })
    })

    it('merge alias attributes', async () => {
      const res = await b.MergeAliasAttributes(123)
      expect(res.amount.value).toEqual(123)
      expect(res.amount.checks['gt_ten'].status).toEqual('succeeded')
    })

    it('return alias with merged attrs', async () => {
      const res = await b.ReturnAliasWithMergedAttributes(123)
      expect(res.value).toEqual(123)
      expect(res.checks['gt_ten'].status).toEqual('succeeded')
    })

    it('alias with multiple attrs', async () => {
      const res = await b.AliasWithMultipleAttrs(123)
      expect(res.value).toEqual(123)
      expect(res.checks['gt_ten'].status).toEqual('succeeded')
    })

    it('simple recursive map alias', async () => {
      const res = await b.SimpleRecursiveMapAlias({ one: { two: { three: {} } } })
      expect(res).toEqual({ one: { two: { three: {} } } })
    })

    it('simple recursive map alias', async () => {
      const res = await b.SimpleRecursiveListAlias([[], [], [[]]])
      expect(res).toEqual([[], [], [[]]])
    })

    it('recursive alias cycles', async () => {
      const res = await b.RecursiveAliasCycle([[], [], [[]]])
      expect(res).toEqual([[], [], [[]]])
    })

    it('json type alias cycle', async () => {
      const data = {
        number: 1,
        string: 'test',
        bool: true,
        list: [1, 2, 3],
        object: { number: 1, string: 'test', bool: true, list: [1, 2, 3] },
        json: {
          number: 1,
          string: 'test',
          bool: true,
          list: [1, 2, 3],
          object: { number: 1, string: 'test', bool: true, list: [1, 2, 3] },
        },
      }
      const res = await b.JsonTypeAliasCycle(data)
      expect(res).toEqual(data)
      expect(res.json.object.list).toEqual([1, 2, 3])
    })
  })

  it('should work for all outputs', async () => {
    const a = 'a' // dummy
    let bool = await b.FnOutputBool(a)
    expect(bool).toEqual(true)

    let int = await b.FnOutputInt(a)
    expect(int).toEqual(5)

    let literal_integer = await b.FnOutputLiteralInt(a)
    expect(literal_integer).toEqual(5)

    let literal_bool = await b.FnOutputLiteralBool(a)
    expect(literal_bool).toEqual(false)

    let literal_string = await b.FnOutputLiteralString(a)
    expect(literal_string).toEqual('example output')

    const list = await b.FnOutputClassList(a)
    expect(list.length).toBeGreaterThan(0)
    assert(list[0].prop1.length > 0)

    const classWEnum = await b.FnOutputClassWithEnum(a)
    expect(['ONE', 'TWO']).toContain(classWEnum.prop2)

    const classs = await b.FnOutputClass(a)
    expect(classs.prop1).not.toBeNull()
    // Actually select 540
    expect(classs.prop2).toEqual(540)

    // enum list output
    const enumList = await b.FnEnumListOutput(a)
    expect(enumList.length).toBe(2)

    const myEnum = await b.FnEnumOutput(a)
  })

  it('works with retries1', async () => {
    try {
      await b.TestRetryConstant()
      assert(false)
    } catch (e) {
      console.log('Expected error', e)
    }
  })

  it('works with retries2', async () => {
    try {
      await b.TestRetryExponential()
      assert(false)
    } catch (e) {
      console.log('Expected error', e)
    }
  })

  it('works with fallbacks', async () => {
    const res = await b.TestFallbackClient()
    expect(res.length).toBeGreaterThan(0)
  })

  it('should work with image from url', async () => {
    let res = await b.TestImageInput(
      Image.fromUrl('https://upload.wikimedia.org/wikipedia/en/4/4d/Shrek_%28character%29.png'),
    )
    expect(res.toLowerCase()).toMatch(/(green|yellow|ogre|shrek)/)
  })

  it('should work with image from base 64', async () => {
    let res = await b.TestImageInput(Image.fromBase64('image/png', image_b64))
    expect(res.toLowerCase()).toMatch(/(green|yellow|ogre|shrek)/)
  })

  it('should work with audio base 64', async () => {
    let res = await b.AudioInput(Audio.fromBase64('audio/mp3', audio_b64))
    expect(res.toLowerCase()).toContain('yes')
  })

  it('should work with audio from url', async () => {
    let res = await b.AudioInput(
      Audio.fromUrl('https://actions.google.com/sounds/v1/emergency/beeper_emergency_call.ogg'),
    )

    expect(res.toLowerCase()).toContain('no')
  })

  it('should support streaming in OpenAI', async () => {
    const stream = b.stream.PromptTestStreaming('Mt Rainier is tall')
    const msgs: string[] = []
    const startTime = performance.now()

    let firstMsgTime: number | null = null
    let lastMsgTime = startTime
    for await (const msg of stream) {
      msgs.push(msg ?? '')
      if (firstMsgTime === null) {
        firstMsgTime = performance.now()
      }
      lastMsgTime = performance.now()
    }
    const final = await stream.getFinalResponse()

    expect(final.length).toBeGreaterThan(0)
    expect(msgs.length).toBeGreaterThan(0)
    expect(firstMsgTime).not.toBeNull()
    expect(firstMsgTime! - startTime).toBeLessThanOrEqual(1500) // 1.5 seconds
    expect(lastMsgTime - startTime).toBeGreaterThan(1000) // 1.0 seconds

    for (let i = 0; i < msgs.length - 2; i++) {
      expect(msgs[i + 1].startsWith(msgs[i])).toBeTruthy()
    }
    expect(msgs.at(-1)).toEqual(final)
  })

  it('should support streaming in Gemini', async () => {
    const stream = b.stream.TestGemini('Dr. Pepper')
    const msgs: string[] = []
    for await (const msg of stream) {
      msgs.push(msg ?? '')
    }
    const final = await stream.getFinalResponse()

    expect(final.length).toBeGreaterThan(0)
    expect(msgs.length).toBeGreaterThan(0)
    for (let i = 0; i < msgs.length - 2; i++) {
      expect(msgs[i + 1].startsWith(msgs[i])).toBeTruthy()
    }
    expect(msgs.at(-1)).toEqual(final)
  }, 20_000)

  it('should support AWS', async () => {
    const res = await b.TestAws('Dr. Pepper')
    expect(res.length).toBeGreaterThan(0)
  })

  it('should support streaming in AWS', async () => {
    const stream = b.stream.TestAws('Dr. Pepper')
    const msgs: string[] = []
    for await (const msg of stream) {
      msgs.push(msg ?? '')
    }
    const final = await stream.getFinalResponse()

    expect(final.length).toBeGreaterThan(0)
    expect(msgs.length).toBeGreaterThan(0)
    for (let i = 0; i < msgs.length - 2; i++) {
      expect(msgs[i + 1].startsWith(msgs[i])).toBeTruthy()
    }
    expect(msgs.at(-1)).toEqual(final)
  })

  it('should allow overriding the region', async () => {
    await expect(async () => {
      await b.TestAwsInvalidRegion('Dr. Pepper')
    }).rejects.toThrow('DispatchFailure')
  })

  it('should support OpenAI shorthand', async () => {
    const res = await b.TestOpenAIShorthand('Dr. Pepper')
    expect(res.length).toBeGreaterThan(0)
  })

  it('should support OpenAI shorthand streaming', async () => {
    const res = await b.stream.TestOpenAIShorthand('Dr. Pepper').getFinalResponse()
    expect(res.length).toBeGreaterThan(0)
  })

  it('should support anthropic shorthand', async () => {
    const res = await b.TestAnthropicShorthand('Dr. Pepper')
    expect(res.length).toBeGreaterThan(0)
  })

  it('should support anthropic shorthand streaming', async () => {
    const res = await b.stream.TestAnthropicShorthand('Dr. Pepper').getFinalResponse()
    expect(res.length).toBeGreaterThan(0)
  })

  it('should support streaming without iterating', async () => {
    const final = await b.stream.PromptTestStreaming('Mt Rainier is tall').getFinalResponse()
    expect(final.length).toBeGreaterThan(0)
  })

  it('should support streaming in Claude', async () => {
    const stream = b.stream.PromptTestClaude('Mt Rainier is tall')
    const msgs: string[] = []
    for await (const msg of stream) {
      msgs.push(msg ?? '')
    }
    const final = await stream.getFinalResponse()

    expect(final.length).toBeGreaterThan(0)
    expect(msgs.length).toBeGreaterThan(0)
    for (let i = 0; i < msgs.length - 2; i++) {
      expect(msgs[i + 1].startsWith(msgs[i])).toBeTruthy()
    }
    expect(msgs.at(-1)).toEqual(final)
  })

  it('should support azure', async () => {
    const res = await b.TestAzure('Donkey Kong')
    expect(res.toLowerCase()).toContain('donkey')
  })

  it('should support azure streaming', async () => {
    const stream = b.stream.TestAzure('Donkey Kong')
    const msgs: string[] = []
    for await (const msg of stream) {
      msgs.push(msg ?? '')
    }
    const final = await stream.getFinalResponse()
    expect(final.length).toBeGreaterThan(0)
  })

  it('should fail if azure is not configured', async () => {
    await expect(async () => {
      await b.TestAzureFailure('Donkey Kong')
    }).rejects.toThrow('BamlClientError')
  })

  // it('should fail if azure is not configured streaming', async () => {
  //   const stream = b.stream.TestAzureFailure('Donkey Kong')
  //   await expect(async () => {
  //     // this should throw an error, not only when we try to get the final response
  //     for await (const msg of stream) {
  //       console.log('msg', msg)
  //     }
  //     // await stream.getFinalResponse()
  //   }).rejects.toThrow('BamlClientError')
  // })

  it('should support vertex', async () => {
    const res = await b.TestVertex('Donkey Kong')
    expect(res.toLowerCase()).toContain('donkey')
  })

  it('supports tracing sync', async () => {
    const blah = 'blah'

    const dummyFunc = (_myArg: string): string => 'hello world'

    const res = traceSync('myFuncParent', (firstArg: string, secondArg: number) => {
      setTags({ myKey: 'myVal' })

      console.log('hello world')

      const res2 = traceSync('dummyFunc', dummyFunc)('dummyFunc')
      console.log('dummyFunc returned')

      const res3 = traceSync('dummyFunc2', dummyFunc)(firstArg)
      console.log('dummyFunc2 returned')

      return 'hello world'
    })('myFuncParent', 10)

    // adding this console log makes it work.
    // console.log('res returned', res)

    traceSync('dummyFunc3', dummyFunc)('hi there')
  })

  // Look at the dashboard to verify results.
  it('supports tracing async', async () => {
    const nestedDummyFn = async (myArg: string): Promise<string> => {
      await scheduler.wait(100) // load-bearing: this ensures that we actually test concurrent execution
      console.log('samDummyNested', myArg)
      return myArg
    }

    const dummyFn = async (myArg: string): Promise<string> => {
      await scheduler.wait(100) // load-bearing: this ensures that we actually test concurrent execution
      const nested = await Promise.all([
        traceAsync('trace:nestedDummyFn1', nestedDummyFn)('nested1'),
        traceAsync('trace:nestedDummyFn2', nestedDummyFn)('nested2'),
        traceAsync('trace:nestedDummyFn3', nestedDummyFn)('nested3'),
      ])
      console.log('dummy', myArg)
      return myArg
    }

    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()
    const _ = DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.drainStats()

    await Promise.all([
      traceAsync('trace:dummyFn1', dummyFn)('hi1'),
      traceAsync('trace:dummyFn2', dummyFn)('hi2'),
      traceAsync('trace:dummyFn3', dummyFn)('hi3'),
    ])

    const res = await traceAsync('parentAsync', async (firstArg: string, secondArg: number) => {
      console.log('hello world')
      setTags({ myKey: 'myVal' })

      const res1 = traceSync('dummyFunc', dummyFn)('firstDummyFuncArg')

      const res2 = await traceAsync('asyncDummyFunc', dummyFn)('secondDummyFuncArg')

      const llm_res = await Promise.all([
        b.TestFnNamedArgsSingleStringList(['a1', 'b', 'c']),
        b.TestFnNamedArgsSingleStringList(['a2', 'b', 'c']),
        b.TestFnNamedArgsSingleStringList(['a3', 'b', 'c']),
      ])

      const res3 = await traceAsync('asyncDummyFunc', dummyFn)('thirdDummyFuncArg')

      return 'hello world'
    })('hi', 10)

    const res2 = await traceAsync('parentAsync2', async (firstArg: string, secondArg: number) => {
      console.log('hello world')

      const syncDummyFn = (_myArg: string): string => 'hello world'
      const res1 = traceSync('dummyFunc', syncDummyFn)('firstDummyFuncArg')

      return 'hello world'
    })('hi', 10)

    DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.flush()
    const stats = DO_NOT_USE_DIRECTLY_UNLESS_YOU_KNOW_WHAT_YOURE_DOING_RUNTIME.drainStats()
    console.log('stats', stats.toJson())
    expect(stats.started).toBe(30)
    expect(stats.finalized).toEqual(stats.started)
    expect(stats.submitted).toEqual(stats.started)
    expect(stats.sent).toEqual(stats.started)
    expect(stats.done).toEqual(stats.started)
    expect(stats.failed).toEqual(0)
  })

  it('should work with dynamic types single', async () => {
    let tb = new TypeBuilder()
    tb.Person.addProperty('last_name', tb.string().optional())
    tb.Person.addProperty('height', tb.float().optional()).description('Height in meters')
    tb.Hobby.addValue('CHESS')
    tb.Hobby.listValues().map(([name, v]) => v.alias(name.toLowerCase()))
    tb.Person.addProperty('hobbies', tb.Hobby.type().list().optional()).description(
      'Some suggested hobbies they might be good at',
    )

    const res = await b.ExtractPeople(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop.",
      { tb },
    )
    expect(res.length).toBeGreaterThan(0)
    console.log(res)
  })

  it('should work with dynamic types enum', async () => {
    let tb = new TypeBuilder()
    const fieldEnum = tb.addEnum('Animal')
    const animals = ['giraffe', 'elephant', 'lion']
    for (const animal of animals) {
      fieldEnum.addValue(animal.toUpperCase())
    }
    tb.Person.addProperty('animalLiked', fieldEnum.type())
    const res = await b.ExtractPeople(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
      { tb },
    )
    expect(res.length).toBeGreaterThan(0)
    expect(res[0]['animalLiked']).toEqual('GIRAFFE')
  })

  it('should work with dynamic literals', async () => {
    let tb = new TypeBuilder()
    const animals = tb.union(['giraffe', 'elephant', 'lion'].map((animal) => tb.literalString(animal.toUpperCase())))
    tb.Person.addProperty('animalLiked', animals)
    const res = await b.ExtractPeople(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
      { tb },
    )
    expect(res.length).toBeGreaterThan(0)
    expect(res[0]['animalLiked']).toEqual('GIRAFFE')
  })

  it('should work with dynamic types class', async () => {
    let tb = new TypeBuilder()
    const animalClass = tb.addClass('Animal')
    animalClass.addProperty('animal', tb.string()).description('The animal mentioned, in singular form.')
    tb.Person.addProperty('animalLiked', animalClass.type())
    const res = await b.ExtractPeople(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop. I like giraffes.",
      { tb },
    )
    expect(res.length).toBeGreaterThan(0)
    const animalLiked = res[0]['animalLiked']
    expect(animalLiked['animal']).toContain('giraffe')
  })

  it('should work with dynamic inputs class', async () => {
    let tb = new TypeBuilder()
    tb.DynInputOutput.addProperty('new-key', tb.string().optional())

    const res = await b.DynamicInputOutput({ 'new-key': 'hi', testKey: 'myTest' }, { tb })
    expect(res['new-key']).toEqual('hi')
    expect(res['testKey']).toEqual('myTest')
  })

  it('should work with dynamic inputs list', async () => {
    let tb = new TypeBuilder()
    tb.DynInputOutput.addProperty('new-key', tb.string().optional())

    const res = await b.DynamicListInputOutput([{ 'new-key': 'hi', testKey: 'myTest' }], { tb })
    expect(res[0]['new-key']).toEqual('hi')
    expect(res[0]['testKey']).toEqual('myTest')
  })

  it('should work with dynamic output map', async () => {
    let tb = new TypeBuilder()
    tb.DynamicOutput.addProperty('hair_color', tb.string())
    tb.DynamicOutput.addProperty('attributes', tb.map(tb.string(), tb.string())).description(
      "Things like 'eye_color' or 'facial_hair'",
    )
    console.log(tb.DynamicOutput.listProperties())
    for (const [prop, _] of tb.DynamicOutput.listProperties()) {
      console.log(`Property: ${prop}`)
    }

    const res = await b.MyFunc(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I have blue eyes and a beard.",
      { tb },
    )

    console.log('final ', res)

    expect(res.hair_color).toEqual('black')
    expect(res.attributes['eye_color']).toEqual('blue')
    expect(res.attributes['facial_hair']).toEqual('beard')
  })

  it('should work with dynamic output union', async () => {
    let tb = new TypeBuilder()
    tb.DynamicOutput.addProperty('hair_color', tb.string())
    tb.DynamicOutput.addProperty('attributes', tb.map(tb.string(), tb.string())).description(
      "Things like 'eye_color' or 'facial_hair'",
    )

    // Define two classes
    const class1 = tb.addClass('Class1')
    class1.addProperty('meters', tb.float())

    const class2 = tb.addClass('Class2')
    class2.addProperty('feet', tb.float())
    class2.addProperty('inches', tb.float().optional())

    // Use the classes in a union property
    tb.DynamicOutput.addProperty('height', tb.union([class1.type(), class2.type()]))
    console.log(tb.DynamicOutput.listProperties())
    for (const [prop, _] of tb.DynamicOutput.listProperties()) {
      console.log(`Property: ${prop}`)
    }

    let res = await b.MyFunc(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I have blue eyes and a beard. I am 30 years old.",
      { tb },
    )

    console.log('final ', res)
    expect(res.hair_color).toEqual('black')
    expect(res.attributes['eye_color']).toEqual('blue')
    expect(res.attributes['facial_hair']).toEqual('beard')
    expect(res.height['feet']).toEqual(6)

    res = await b.MyFunc(
      "My name is Harrison. My hair is black and I'm 1.8 meters tall. I have blue eyes and a beard. I am 30 years old.",
      { tb },
    )

    console.log('final ', res)
    expect(res.hair_color).toEqual('black')
    expect(res.attributes['eye_color']).toEqual('blue')
    expect(res.attributes['facial_hair']).toEqual('beard')
    expect(res.height['meters']).toEqual(1.8)
  })

  // test with extra list, boolean in the input as well.

  it('should work with nested classes', async () => {
    let stream = b.stream.FnOutputClassNested('hi!')
    let msgs: RecursivePartialNull<TestClassNested[]> = []
    for await (const msg of stream) {
      console.log('msg', msg)
      msgs.push(msg)
    }

    const final = await stream.getFinalResponse()
    expect(msgs.length).toBeGreaterThan(0)
    expect(msgs.at(-1)).toEqual(final)
  })

  it('should work with dynamic client', async () => {
    const clientRegistry = new ClientRegistry()
    clientRegistry.addLlmClient('myClient', 'openai', {
      model: 'gpt-3.5-turbo',
    })
    clientRegistry.setPrimary('myClient')

    const capitol = await b.ExpectFailure({
      clientRegistry,
    })
    expect(capitol.toLowerCase()).toContain('london')
  })

  it("should work with 'onLogEvent'", async () => {
    flush() // Wait for all logs to be sent so no calls to onLogEvent are missed.
    onLogEvent((param2) => {
      console.log('onLogEvent', param2)
    })
    const res = await b.TestFnNamedArgsSingleStringList(['a', 'b', 'c'])
    expect(res).toContain('a')
    const res2 = await b.TestFnNamedArgsSingleStringList(['d', 'e', 'f'])
    expect(res2).toContain('d')
    flush() // Wait for all logs to be sent so no calls to onLogEvent are missed.
    onLogEvent(undefined)
  })

  it('should work with a sync client', () => {
    const res = b_sync.TestFnNamedArgsSingleStringList(['a', 'b', 'c'])
    expect(res).toContain('a')
  })

  it('should raise an error when appropriate', async () => {
    await expect(async () => {
      await b.TestCaching(111 as unknown as string, 'fiction') // intentionally passing an int instead of a string
    }).rejects.toThrow('BamlInvalidArgumentError')

    await expect(async () => {
      const cr = new ClientRegistry()
      cr.addLlmClient('MyClient', 'openai', { model: 'gpt-4o-mini', api_key: 'INVALID_KEY' })
      cr.setPrimary('MyClient')
      await b.MyFunc("My name is Harrison. My hair is black and I'm 6 feet tall.", { clientRegistry: cr })
    }).rejects.toThrow('BamlClientError')

    try {
      const cr = new ClientRegistry()
      cr.addLlmClient('MyClient', 'openai', { model: 'gpt-4o-mini', api_key: 'INVALID_KEY' })
      cr.setPrimary('MyClient')
      await b.MyFunc("My name is Harrison. My hair is black and I'm 6 feet tall.", { clientRegistry: cr })
      fail('Expected b.MyFunc to throw a BamlClientHttpError')
    } catch (error: any) {
      console.log('Error:', error)
      expect(error.message).toContain('BamlClientHttpError')
    }

    await expect(async () => {
      try {
        await b.DummyOutputFunction('dummy input')
      } catch (error) {
        console.log(error)
        throw error
      }
    }).rejects.toThrow('BamlValidationError')
  })

  it('should raise a BAMLValidationError', async () => {
    try {
      await b.DummyOutputFunction('dummy input')
      fail('Expected b.DummyOutputFunction to throw a BamlValidationError')
    } catch (error: any) {
      if (error instanceof BamlValidationError) {
        console.log('error', error)
        expect(error.message).toContain('BamlValidationError')
        expect(error.prompt).toContain('Say "hello there".')
        expect(error.raw_output).toBeDefined()
        expect(error.raw_output.length).toBeGreaterThan(0)
      } else {
        fail('Expected error to be an instance of BamlValidationError')
      }
    }
  })

  it('should reset environment variables correctly', async () => {
    const envVars = {
      OPENAI_API_KEY: 'sk-1234567890',
    }
    resetBamlEnvVars(envVars)

    const topLevelSyncTracing = traceSync('name', () => {
      resetBamlEnvVars(envVars)
    })

    const atopLevelAsyncTracing = traceAsync('name', async () => {
      resetBamlEnvVars(envVars)
    })

    await expect(async () => {
      topLevelSyncTracing()
    }).rejects.toThrow('BamlError')

    await expect(async () => {
      await atopLevelAsyncTracing()
    }).rejects.toThrow('BamlError')

    await expect(async () => {
      await b.ExtractPeople(
        "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop.",
      )
    }).rejects.toThrow('BamlClientHttpError')

    resetBamlEnvVars(
      Object.fromEntries(Object.entries(process.env).filter(([_, v]) => v !== undefined)) as Record<string, string>,
    )
    const people = await b.ExtractPeople(
      "My name is Harrison. My hair is black and I'm 6 feet tall. I'm pretty good around the hoop.",
    )
    expect(people.length).toBeGreaterThan(0)
  })

  it('should use aliases when serializing input objects - classes', async () => {
    const res = await b.AliasedInputClass({ key: 'hello', key2: 'world' })
    expect(res).toContain('color')

    const res2 = await b.AliasedInputClassNested({
      key: 'hello',
      nested: { key: 'nested-hello', key2: 'nested-world' },
    })
    expect(res2).toContain('interesting-key')
  })

  it('should use aliases when serializing, but still have original keys in jinja', async () => {
    const res = await b.AliasedInputClass2({ key: 'tiger', key2: 'world' })
    expect(res).toContain('tiger')

    const res2 = await b.AliasedInputClassNested({
      key: 'hello',
      nested: { key: 'nested-hello', key2: 'nested-world' },
    })
    expect(res2).toContain('interesting-key')
  })

  // TODO: Enum aliases are not supported
  it('should use aliases when serializing input objects - enums', async () => {
    const res = await b.AliasedInputEnum(AliasedEnum.KEY_ONE)
    expect(res).not.toContain('tiger')
  })

  // TODO: enum aliases are not supported
  it('should use aliases when serializing input objects - lists', async () => {
    const res = await b.AliasedInputList([AliasedEnum.KEY_ONE, AliasedEnum.KEY_TWO])
    expect(res).not.toContain('tiger')
  })

  it('constraints: should handle checks in return types', async () => {
    const res = await b.PredictAge('Greg')
    expect(res.certainty.checks.unreasonably_certain.status).toBe('failed')
  })

  it('constraints: should handle checks in returned unions', async () => {
    const res = await b.ExtractContactInfo('Reach me at 111-222-3333, or robert@boundaryml.com if needed')
    expect(res.primary.value).toBe('111-222-3333')
    expect(res.secondary?.value).toBe('robert@boundaryml.com')
  })

  it('constraints: should handle block-level checks', async () => {
    const res = await b.MakeBlockConstraint()
    expect(res.checks.cross_field.status).toBe('failed')
  })

  it('constraints: should handle nested-block-level checks', async () => {
    const res = await b.MakeNestedBlockConstraint()
    console.log(JSON.stringify(res))
    expect(res.nbc.checks.cross_field.status).toBe('succeeded')
  })

  it('simple recursive type', async () => {
    const res = await b.BuildLinkedList([1, 2, 3, 4, 5])
    expect(res).toEqual({
      head: {
        data: 1,
        next: {
          data: 2,
          next: {
            data: 3,
            next: {
              data: 4,
              next: {
                data: 5,
                next: null,
              },
            },
          },
        },
      },
      len: 5,
    })
  })

  it('mutually recursive type', async () => {
    const res = await b.BuildTree({
      data: 5,
      left: {
        data: 3,
        left: {
          data: 1,
          left: null,
          right: {
            data: 2,
            left: null,
            right: null,
          },
        },
        right: {
          data: 4,
          left: null,
          right: null,
        },
      },
      right: {
        data: 7,
        left: {
          data: 6,
          left: null,
          right: null,
        },
        right: {
          data: 8,
          left: null,
          right: null,
        },
      },
    })
    expect(res).toEqual({
      data: 5,
      children: {
        trees: [
          {
            data: 3,
            children: {
              trees: [
                {
                  data: 1,
                  children: {
                    trees: [
                      {
                        data: 2,
                        children: {
                          trees: [],
                        },
                      },
                    ],
                  },
                },
                {
                  data: 4,
                  children: {
                    trees: [],
                  },
                },
              ],
            },
          },
          {
            data: 7,
            children: {
              trees: [
                {
                  data: 6,
                  children: {
                    trees: [],
                  },
                },
                {
                  data: 8,
                  children: {
                    trees: [],
                  },
                },
              ],
            },
          },
        ],
      },
    })
  })

  afterAll(async () => {
    flush()
  })
})
