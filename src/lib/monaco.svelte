<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let value = '';
  
  let box;
  let ed;
  let ready = false;
  let provider;

  async function ask(method, params) {
    try {
      const res = await invoke('lsp_req', {
        method,
        params: JSON.stringify(params)
      });
      return JSON.parse(res);
    } catch (e) {
      console.error('LSP error:', method, e);
      return null;
    }
  }

  async function tell(method, params) {
    try {
      await invoke('lsp_notif', {
        method,
        params: JSON.stringify(params)
      });
    } catch (e) {
      console.error('LSP notif error:', method, e);
    }
  }

  const basic = [
    { label: 'game', kind: 6, insertText: 'game', detail: 'The game object' },
    { label: 'workspace', kind: 6, insertText: 'workspace', detail: 'The workspace' },
    { label: 'Players', kind: 6, insertText: 'game:GetService("Players")', detail: 'Players service' },
    { label: 'ReplicatedStorage', kind: 6, insertText: 'game:GetService("ReplicatedStorage")', detail: 'ReplicatedStorage service' },
    { label: 'ServerStorage', kind: 6, insertText: 'game:GetService("ServerStorage")', detail: 'ServerStorage service' },
    { label: 'RunService', kind: 6, insertText: 'game:GetService("RunService")', detail: 'RunService' },
  ];

  onMount(() => {
    if (window.require) {
      window.require.config({ 
        paths: { vs: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs' }
      });
      
      window.require(['vs/editor/editor.main'], async () => {
        window.monaco.editor.defineTheme('custom', {
          base: 'vs-dark',
          inherit: true,
          rules: [],
          colors: {
            'editor.background': '#0d0d0d'
          }
        });

        const init = await ask('initialize', {
          processId: null,
          rootUri: null,
          capabilities: {
            textDocument: {
              completion: {
                completionItem: {
                  snippetSupport: true
                }
              },
              publishDiagnostics: {}
            }
          }
        });

        if (init?.result) {
          await tell('initialized', {});
          ready = true;
        }

        if (!window.monacoProviderRegistered) {
          provider = window.monaco.languages.registerCompletionItemProvider('lua', {
            provideCompletionItems: async (model, pos) => {
              const basics = basic.map(item => ({
                label: item.label,
                kind: item.kind,
                insertText: item.insertText,
                detail: item.detail
              }));

              if (!ready) {
                return { suggestions: basics };
              }

              const res = await ask('textDocument/completion', {
                textDocument: { uri: model.uri.toString() },
                position: { line: pos.lineNumber - 1, character: pos.column - 1 }
              });

              if (!res?.result) return { suggestions: basics };

              const items = Array.isArray(res.result) ? res.result : res.result.items || [];
              
              const lsp = items.map(item => {
                const isFunc = item.kind === window.monaco.languages.CompletionItemKind.Function || 
                               item.kind === window.monaco.languages.CompletionItemKind.Method;
                
                let text = item.insertText || item.label;
                let isSnippet = item.insertTextFormat === 2;
                
                if (isFunc && !text.includes('(') && !isSnippet) {
                  text = `${text}($0)`;
                  isSnippet = true;
                }
                
                return {
                  label: item.label,
                  kind: item.kind || window.monaco.languages.CompletionItemKind.Function,
                  insertText: text,
                  insertTextRules: isSnippet ? window.monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet : undefined,
                  detail: item.detail,
                  documentation: item.documentation?.value || item.documentation
                };
              });

              return { suggestions: [...basics, ...lsp] };
            }
          });
          
          window.monacoProviderRegistered = true;
        }

        if (!value) {
          value = '--[[ voxlis ]]';
        }
        
        ed = window.monaco.editor.create(box, {
          value: value,
          language: 'lua',
          theme: 'custom',
          automaticLayout: true,
          minimap: { enabled: true },
          fontSize: 15,
          padding: { top: 10 },
        });

        await tell('textDocument/didOpen', {
          textDocument: {
            uri: ed.getModel().uri.toString(),
            languageId: 'luau',
            version: 1,
            text: value
          }
        });

        let diagTimeout;
        ed.onDidChangeModelContent(async () => {
          value = ed.getValue();
          if (ready) {
            await tell('textDocument/didChange', {
              textDocument: { 
                uri: ed.getModel().uri.toString(),
                version: ed.getModel().getVersionId()
              },
              contentChanges: [{ text: value }]
            });

            clearTimeout(diagTimeout);
            diagTimeout = setTimeout(async () => {
              const diag = await ask('textDocument/diagnostic', {
                textDocument: { uri: ed.getModel().uri.toString() }
              });

              if (diag?.result?.items) {
                const marks = diag.result.items.map(d => ({
                  severity: d.severity === 1 ? window.monaco.MarkerSeverity.Error : 
                           d.severity === 2 ? window.monaco.MarkerSeverity.Warning : 
                           window.monaco.MarkerSeverity.Info,
                  startLineNumber: d.range.start.line + 1,
                  startColumn: d.range.start.character + 1,
                  endLineNumber: d.range.end.line + 1,
                  endColumn: d.range.end.character + 1,
                  message: d.message
                }));
                window.monaco.editor.setModelMarkers(ed.getModel(), 'luau', marks);
              }
            }, 500);
          }
        });
      });
    }
  });

  onDestroy(() => {
    if (ed) {
      ed.dispose();
    }
  });
</script>

<div bind:this={box} class="w-full h-full"></div>