<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { s } from './data.ts';

let { value = $bindable('') } = $props();

let box: HTMLDivElement;
let editor: any;
let mon: any;
let initialized = false;
let log = $state<string[]>([]);
const L = (m: string) => { log = [...log, `${new Date().toISOString().slice(11,23)} ${m}`]; console.log('[monaco]', m); };

const cssVar = (n: string) => getComputedStyle(document.documentElement).getPropertyValue(n).trim();

const ruleColor = (n: string) => {
  const v = cssVar(n).replace('#', '');
  if (v.length === 3) return v[0]+v[0]+v[1]+v[1]+v[2]+v[2];
  return v.length === 6 ? v : 'ffffff';
};

const themeColor = (n: string, fallback: string) => {
  const v = cssVar(n);
  const hex = v.replace('#', '');
  if (hex.length === 3) return '#' + hex[0]+hex[0]+hex[1]+hex[1]+hex[2]+hex[2];
  if (hex.length === 6 || hex.length === 8) return '#' + hex;
  return fallback;
};

function load() {
  L(`load() called | window.monaco=${!!(window as any).monaco} | window.require=${!!(window as any).require}`);
  
  if ((window as any).monaco) { L('monaco already on window, calling init'); init(); return; }

  L('injecting script tag into head');
  const script = document.createElement('script');
  script.src = 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js';
  script.crossOrigin = 'anonymous';
  
  script.onerror = (e) => L(`script onerror: ${JSON.stringify(e)}`);
  
  script.onload = () => {
    L(`script onload | window.require=${!!(window as any).require}`);
    try {
      const req = (window as any).require;
      if (!req) { L('ERROR: require is null after script load'); return; }
      req.config({ paths: { vs: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs' }});
      L('require.config done, calling require([vs/editor/editor.main])');
      req(['vs/editor/editor.main'], 
        () => { L('require callback success'); init(); },
        (err: any) => L(`require callback error: ${err}`)
      );
    } catch(e) { L(`onload catch: ${e}`); }
  };
  
  document.head.appendChild(script);
  L('script tag appended');
}

function init() {
  L('init() start');
  try {
    mon = (window as any).monaco;
    if (!mon) { L('ERROR: window.monaco is null in init'); return; }
    L('monaco found, defining theme');

    mon.editor.defineTheme('juju-dynamic', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'keyword',    foreground: ruleColor('--monaco-keyword') },
        { token: 'string',     foreground: ruleColor('--monaco-string') },
        { token: 'number',     foreground: ruleColor('--monaco-number') },
        { token: 'comment',    foreground: ruleColor('--monaco-comment'), fontStyle: 'italic' },
        { token: 'identifier', foreground: ruleColor('--monaco-identifier') },
        { token: 'delimiter',  foreground: ruleColor('--monaco-delimiter') },
        { token: 'operator',   foreground: ruleColor('--monaco-delimiter') },
      ],
colors: {
  'editor.background':                                themeColor('--monaco-bg', '#000000'),
  'editor.lineHighlightBackground':                   '#00000000',
  'editor.lineHighlightBorder':                       '#00000000',
  'editorLineNumber.foreground':                      themeColor('--monaco-line-number', '#ffffff40'),
  'editorLineNumber.activeForeground':                themeColor('--monaco-line-number-active', '#ffffff80'),
  'editorCursor.foreground':                          themeColor('--monaco-keyword', '#ffffff'),
  'editor.selectionBackground':                       themeColor('--monaco-selection', '#252525'),
  'scrollbarSlider.background':                       themeColor('--monaco-scrollbar', '#0D0D0D'),
  'scrollbarSlider.hoverBackground':                  themeColor('--monaco-scrollbar-hover', '#1A1A1A'),
  'editorOverviewRuler.border':                       '#00000000',
  'editorOverviewRuler.selectionHighlightForeground': '#00000000',
  'editorOverviewRuler.wordHighlightForeground':      '#00000000',
  'editorOverviewRuler.wordHighlightStrongForeground':'#00000000',
  'editorOverviewRuler.findMatchForeground':          '#00000000',
}
    });
    L('theme defined, creating editor');

    let cur: any;
    s.subscribe(v => cur = v);

    editor = mon.editor.create(box, {
      value: value || '',
      language: 'lua',
      theme: 'juju-dynamic',
      automaticLayout: true,
      fontSize: cur?.fontSize,
      minimap: { enabled: cur?.minimap },
      hideCursorInOverviewRuler: true,
      overviewRulerBorder: false,
      scrollbar: { verticalScrollbarSize: 0, horizontalScrollbarSize: 8, useShadows: false }
    });

    L('editor created');
    editor.onDidChangeModelContent(() => { value = editor.getValue(); });
    s.subscribe(v => editor?.updateOptions({ fontSize: v.fontSize, minimap: { enabled: v.minimap } }));
    initialized = true;
    L('done');
    requestAnimationFrame(() => editor?.layout());
  } catch(e) { L(`init catch: ${e}`); }
}

onMount(() => { L('onMount'); load(); });

$effect(() => {
  void value, initialized;
  if (!initialized || !editor || value === undefined) return;
  if (editor.getValue() !== value) editor.setValue(value || '');
});

export function forceLayout() { requestAnimationFrame(() => editor?.layout()); }
onDestroy(() => editor?.dispose());
</script>

<div bind:this={box} class="w-full h-full relative z-10"></div>
<div class="absolute inset-0 z-0 p-2 text-[10px] font-mono text-white pointer-events-none {initialized ? 'hidden' : ''}" style="background:rgba(0,0,0,0.85)">
  {#each log as line}<div>{line}</div>{/each}
</div>