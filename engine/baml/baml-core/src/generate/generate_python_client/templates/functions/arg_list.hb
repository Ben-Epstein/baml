{{#if unnamed_args}}
arg: {{args.[0].type}}, /{{else}}
{{#each args}}{{this.name}}: {{this.type}}{{#unless @last}}, {{/unless}}{{/each}}{{/if}}