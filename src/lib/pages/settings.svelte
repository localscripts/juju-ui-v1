<script>
  import Search from '../settings/search.svelte';
  import Interface from '../settings/interface.svelte';
  import Core from '../settings/core.svelte';
  import Editor from '../settings/editor.svelte';
  import Themes from '../settings/themes.svelte';

  let { onpreview = () => {}, onimport = () => {}, onexport = () => {}, onreset = () => {}, bindreset = () => {}, bindimport = () => {} } = $props();

  let query = $state("");
  let filters = $state([]);
  let selected = $state("interface");

  const tabs = [
    {
      id: 'interface', label: 'INTERFACE', viewBox: '0 0 24 24',
      grad: { x1: '12', y1: '0', x2: '12', y2: '24' }, scale: '',
      paths: [
        `<path d="M3 2.25C2.58579 2.25 2.25 2.58579 2.25 3V8H21.75V3C21.75 2.58579 21.4142 2.25 21 2.25H3Z"/>`,
        `<path d="M21.75 10H13V21.75H21C21.4142 21.75 21.75 21.4142 21.75 21V10Z"/>`,
        `<path d="M11 21.75V10H2.25V21C2.25 21.4142 2.58579 21.75 3 21.75H11Z"/>`
      ]
    },
    {
      id: 'core', label: 'CORE', viewBox: '0 0 24 24',
      grad: { x1: '12', y1: '0', x2: '12', y2: '24' }, scale: '',
      paths: [
        `<path d="M17.5 1.25C17.771 1.25 18.0208 1.39615 18.1537 1.6323L22.6537 9.6323C22.7843 9.86453 22.7819 10.1486 22.6474 10.3786C22.5129 10.6086 22.2664 10.75 22 10.75H13C12.7336 10.75 12.4871 10.6086 12.3526 10.3786C12.2181 10.1486 12.2157 9.86453 12.3463 9.6323L16.8463 1.6323C16.9792 1.39615 17.229 1.25 17.5 1.25Z"/>`,
        `<path d="M1.25 2C1.25 1.58579 1.58579 1.25 2 1.25H10C10.4142 1.25 10.75 1.58579 10.75 2V10C10.75 10.4142 10.4142 10.75 10 10.75H2C1.58579 10.75 1.25 10.4142 1.25 10V2Z"/>`,
        `<path d="M12.75 18C12.75 15.3766 14.8766 13.25 17.5 13.25C20.1234 13.25 22.25 15.3766 22.25 18C22.25 20.6234 20.1234 22.75 17.5 22.75C14.8766 22.75 12.75 20.6234 12.75 18Z"/>`,
        `<path fill-rule="evenodd" clip-rule="evenodd" d="M5.95711 16.6286L9.24994 13.3358L10.6642 14.75L7.37132 18.0429L10.6642 21.3358L9.25 22.75L5.95711 19.4571L2.66422 22.75L1.25 21.3358L4.5429 18.0429L1.25007 14.75L2.66428 13.3358L5.95711 16.6286Z"/>`
      ]
    },
    {
      id: 'editor', label: 'EDITOR', viewBox: '0 0 110 110',
      grad: { x1: '55', y1: '0', x2: '55', y2: '110' }, scale: 'scale(0.82) translate(10, 10)',
      paths: [
        `<g transform="translate(-6,-6)"><path d="M 5,5 L 55,5 L 55,22 C 43,22 43,38 55,38 L 55,55 L 38,55 C 38,67 22,67 22,55 L 5,55 Z"/></g>`,
        `<g transform="translate(6,-6)"><path d="M 55,5 L 105,5 L 105,55 L 88,55 C 88,43 72,43 72,55 L 55,55 L 55,38 C 43,38 43,22 55,22 Z"/></g>`,
        `<g transform="translate(-6,6)"><path d="M 5,55 L 22,55 C 22,67 38,67 38,55 L 55,55 L 55,72 C 67,72 67,88 55,88 L 55,105 L 5,105 Z"/></g>`,
        `<g transform="translate(6,6)"><path d="M 55,55 L 72,55 C 72,43 88,43 88,55 L 105,55 L 105,105 L 55,105 L 55,88 C 67,88 67,72 55,72 Z"/></g>`
      ]
    },
    {
      id: 'themes', label: 'THEMES', viewBox: '0 0 24 24',
      grad: { x1: '12', y1: '0', x2: '12', y2: '24' }, scale: '',
      paths: [`<path fill-rule="evenodd" clip-rule="evenodd" d="M10.19 1.74902H1.74884V10.1906L10.19 1.74902ZM1.74884 12.3119V15.6917L15.6901 1.74902H12.3113L1.74884 12.3119ZM17.8113 1.74902L1.74884 17.8131V21.1917L21.1901 1.74902H17.8113ZM22.2488 2.81159L2.81285 22.249H6.1916L22.2488 6.19021V2.81159ZM22.2488 8.31164L8.31282 22.249H11.6916L22.2488 11.6902V8.31164ZM22.2488 13.8117L13.8127 22.249H22.2488V13.8117Z"/>`]
    }
  ];

  const filterOpts = ['checkbox', 'button', 'dropdown', 'slider', 'textbox'];
  let filterOpen = $state(false);
  function toggleFilter(f) {
    filters = filters.includes(f) ? filters.filter(x => x !== f) : [...filters, f];
  }
</script>

<svelte:window onclick={() => filterOpen = false} />

<div class="flex w-full h-full">
  <div class="w-50 min-w-50 flex flex-col" style="height:100%;background:var(--settings-sidebar-bg)">
    <div class="p-3 pb-3 flex flex-col gap-4">
      <Search bind:value={query} bind:filterOpen bind:filters {toggleFilter} {filterOpts} />
      <div class="font-bold text-[15px]" style="color:var(--settings-sidebar-label)">SETTINGS</div>
    </div>
    <div class="flex flex-col gap-2 px-3 overflow-y-auto" style="flex:1;scrollbar-width:thin;scrollbar-color:var(--scrollbar-thumb) transparent">
      {#each tabs as tab}
        {@const active = selected === tab.id}
        <button onclick={() => selected = tab.id} class="w-full p-2 rounded-md font-[565] flex gap-2.5 items-center"
          style={active ? 'background:linear-gradient(to bottom,var(--settings-btn-selected-from),var(--settings-btn-selected-to))' : 'background:var(--settings-btn-unselected)'}
          onmouseenter={(e) => { if (!active) e.currentTarget.style.background = 'var(--settings-sidebar-item-hover)'; }}
          onmouseleave={(e) => { if (!active) e.currentTarget.style.background = 'var(--settings-btn-unselected)'; }}>
          <svg class="size-5 flex-shrink-0" viewBox={tab.viewBox} fill={active ? `url(#${tab.id}_grad)` : 'var(--settings-icon-unselected)'}>
            <g transform={tab.scale}>{#each tab.paths as p}{@html p}{/each}</g>
            {#if active}
              <defs>
                <linearGradient id="{tab.id}_grad" x1={tab.grad.x1} y1={tab.grad.y1} x2={tab.grad.x2} y2={tab.grad.y2} gradientUnits="userSpaceOnUse">
                  <stop offset="0%" stop-color="var(--settings-icon-selected-top)"/>
                  <stop offset="100%" stop-color="var(--settings-icon-selected-bottom)"/>
                </linearGradient>
              </defs>
            {/if}
          </svg>
          {#if active}
            <svg class="flex-1 h-[1.5em]" viewBox="0 0 200 30" preserveAspectRatio="xMinYMid meet">
              <defs>
                <linearGradient id="{tab.id}_text_grad" x1="0" y1="0" x2="0" y2="30" gradientUnits="userSpaceOnUse">
                  <stop offset="0%" stop-color="var(--settings-text-selected-top)"/>
                  <stop offset="100%" stop-color="var(--settings-text-selected-bottom)"/>
                </linearGradient>
              </defs>
              <text x="0" y="22" font-size="24" font-weight="565" fill="url(#{tab.id}_text_grad)" font-family="Space Grotesk, sans-serif">{tab.label}</text>
            </svg>
          {:else}
            <span class="flex-1 text-left" style="color:var(--settings-text-unselected)">{tab.label}</span>
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <div class="flex-1 h-full" style="background:var(--settings-content-bg)">
    <div class:hidden={selected !== 'interface'} class="w-full h-full"><Interface {query} {filters} ongoto={() => selected = 'themes'} /></div>
    <div class:hidden={selected !== 'core'} class="w-full h-full"><Core {query} {filters} /></div>
    <div class:hidden={selected !== 'editor'} class="w-full h-full"><Editor {query} {filters} /></div>
    <div class:hidden={selected !== 'themes'} class="w-full h-full">
      <Themes {query} {filters} {onpreview} {onimport} {onexport} {onreset} {bindreset} {bindimport} />
    </div>
  </div>
</div>