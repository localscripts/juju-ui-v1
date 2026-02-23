<script>
  let { onpage = () => {}, activePage = 'editor', onnews = () => {} } = $props();
</script>

<div data-tauri-drag-region
  class="w-16 fixed h-full text-white gap-6 flex flex-col items-center"
  style="background: var(--leftbar-bg);"
>
  <div class="flex gap-3.75 mt-3.75 flex-col items-center flex-1">

    <button onclick={() => onpage('editor')} class="size-10 rounded-md"
      style={activePage === 'editor' ? 'background: linear-gradient(to bottom, var(--leftbar-btn-active-from), var(--leftbar-btn-active-to));' : ''}
      onmouseenter={(e) => { if (activePage !== 'editor') e.currentTarget.style.background = 'var(--leftbar-btn-hover)'; }}
      onmouseleave={(e) => { if (activePage !== 'editor') e.currentTarget.style.background = ''; }}
    >
      <div class="w-full rounded-md justify-center h-full flex items-center">
        {#if activePage === 'editor'}
          <svg class="h-6 w-6" viewBox="0 0 24 24" fill="url(#code_grad_on)">
            <path d="M2.25 2.25H13.25V9.25H2.25V2.25Z"/>
            <path d="M14.75 2.25H21.75V13.25H14.75V2.25Z"/>
            <path d="M14.75 14.75H21.75V21.75H14.75V14.75Z"/>
            <path d="M2.25 10.75H13.25V21.75H2.25V10.75Z"/>
            <defs>
              <linearGradient id="code_grad_on" x1="12" y1="0" x2="12" y2="24" gradientUnits="userSpaceOnUse">
                <stop offset="0%" stop-color="var(--icon-leftbar-selected-top)"/>
                <stop offset="100%" stop-color="var(--icon-leftbar-selected-bottom)"/>
              </linearGradient>
            </defs>
          </svg>
        {:else}
          <svg class="h-6 w-6" viewBox="0 0 24 24" fill="var(--icon-leftbar-unselected)">
            <path d="M2.25 2.25H13.25V9.25H2.25V2.25Z"/>
            <path d="M14.75 2.25H21.75V13.25H14.75V2.25Z"/>
            <path d="M14.75 14.75H21.75V21.75H14.75V14.75Z"/>
            <path d="M2.25 10.75H13.25V21.75H2.25V10.75Z"/>
          </svg>
        {/if}
      </div>
    </button>

    <button onclick={() => onpage('cloud')} class="size-10 rounded-md"
      style={activePage === 'cloud' ? 'background: linear-gradient(to bottom, var(--leftbar-btn-active-from), var(--leftbar-btn-active-to));' : ''}
      onmouseenter={(e) => { if (activePage !== 'cloud') e.currentTarget.style.background = 'var(--leftbar-btn-hover)'; }}
      onmouseleave={(e) => { if (activePage !== 'cloud') e.currentTarget.style.background = ''; }}
    >
      <div class="w-full rounded-md justify-center h-full flex items-center">
        {#if activePage === 'cloud'}
          <svg class="size-6" viewBox="0 0 24 24" fill="url(#cloud_grad_on)">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M22.75 10.25H1.25022L1.25 21.75H22.75V10.25ZM9.87422 16.2492L8.14922 13.9492L6.94922 14.8492L9.12422 17.7492H14.8742L17.0492 14.8492L15.8492 13.9492L14.1242 16.2492H9.87422Z"/>
            <path d="M20.7499 6.25H3.25011L3.25 9H20.7499V6.25Z"/>
            <path d="M18.7499 2.25H5.25008L5.25 5H18.7499V2.25Z"/>
            <defs>
              <linearGradient id="cloud_grad_on" x1="12" y1="0" x2="12" y2="24" gradientUnits="userSpaceOnUse">
                <stop offset="0%" stop-color="var(--icon-leftbar-selected-top)"/>
                <stop offset="100%" stop-color="var(--icon-leftbar-selected-bottom)"/>
              </linearGradient>
            </defs>
          </svg>
        {:else}
          <svg class="size-6" viewBox="0 0 24 24" fill="var(--icon-leftbar-unselected)">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M22.75 10.25H1.25022L1.25 21.75H22.75V10.25ZM9.87422 16.2492L8.14922 13.9492L6.94922 14.8492L9.12422 17.7492H14.8742L17.0492 14.8492L15.8492 13.9492L14.1242 16.2492H9.87422Z"/>
            <path d="M20.7499 6.25H3.25011L3.25 9H20.7499V6.25Z"/>
            <path d="M18.7499 2.25H5.25008L5.25 5H18.7499V2.25Z"/>
          </svg>
        {/if}
      </div>
    </button>

    <button onclick={() => onpage('settings')} class="size-10 rounded-md"
      style={activePage === 'settings' ? 'background: linear-gradient(to bottom, var(--leftbar-btn-active-from), var(--leftbar-btn-active-to));' : ''}
      onmouseenter={(e) => { if (activePage !== 'settings') e.currentTarget.style.background = 'var(--leftbar-btn-hover)'; }}
      onmouseleave={(e) => { if (activePage !== 'settings') e.currentTarget.style.background = ''; }}
    >
      <div class="w-full rounded-md justify-center h-full flex items-center">
        {#if activePage === 'settings'}
          <svg class="size-6" viewBox="0 0 24 24" fill="url(#settings_grad_on)">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M9.04738 1.25C8.63317 1.25 8.29738 1.58579 8.29738 2V4.15233L7.1544 4.82396L5.33215 3.75501C5.15903 3.65346 4.95244 3.62553 4.75857 3.67747C4.56471 3.72941 4.39975 3.85689 4.30059 4.03139L1.34792 9.22754C1.14527 9.58417 1.26671 10.0374 1.62052 10.245L3.45152 11.3191L3.45152 12.681L1.62062 13.755C1.26681 13.9626 1.14537 14.4158 1.34802 14.7725L4.30069 19.9686C4.39985 20.1431 4.5648 20.2706 4.75867 20.3225C4.95254 20.3745 5.15913 20.3465 5.33225 20.245L7.15445 19.1761L8.29738 19.8477V22C8.29738 22.4142 8.63317 22.75 9.04738 22.75H14.9527C15.3669 22.75 15.7027 22.4142 15.7027 22V19.8477L16.8456 19.1761L18.6677 20.245C18.8409 20.3465 19.0475 20.3745 19.2413 20.3225C19.4352 20.2706 19.6002 20.1431 19.6993 19.9686L22.652 14.7725C22.8546 14.4158 22.7332 13.9626 22.3794 13.755L20.5486 12.681L20.5486 11.319L22.3795 10.245C22.7333 10.0374 22.8547 9.58417 22.6521 9.22754L19.6994 4.03139C19.6003 3.85689 19.4353 3.72941 19.2414 3.67747C19.0476 3.62553 18.841 3.65346 18.6678 3.75501L16.8456 4.82393L15.7027 4.15233V2C15.7027 1.58579 15.3669 1.25 14.9527 1.25H9.04738ZM11.9999 15.5C13.9329 15.5 15.4999 13.933 15.4999 12C15.4999 10.067 13.9329 8.5 11.9999 8.5C10.0669 8.5 8.49988 10.067 8.49988 12C8.49988 13.933 10.0669 15.5 11.9999 15.5Z"/>
            <defs>
              <linearGradient id="settings_grad_on" x1="12" y1="0" x2="12" y2="24" gradientUnits="userSpaceOnUse">
                <stop offset="0%" stop-color="var(--icon-leftbar-selected-top)"/>
                <stop offset="100%" stop-color="var(--icon-leftbar-selected-bottom)"/>
              </linearGradient>
            </defs>
          </svg>
        {:else}
          <svg class="size-6" viewBox="0 0 24 24" fill="var(--icon-leftbar-unselected)">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M9.04738 1.25C8.63317 1.25 8.29738 1.58579 8.29738 2V4.15233L7.1544 4.82396L5.33215 3.75501C5.15903 3.65346 4.95244 3.62553 4.75857 3.67747C4.56471 3.72941 4.39975 3.85689 4.30059 4.03139L1.34792 9.22754C1.14527 9.58417 1.26671 10.0374 1.62052 10.245L3.45152 11.3191L3.45152 12.681L1.62062 13.755C1.26681 13.9626 1.14537 14.4158 1.34802 14.7725L4.30069 19.9686C4.39985 20.1431 4.5648 20.2706 4.75867 20.3225C4.95254 20.3745 5.15913 20.3465 5.33225 20.245L7.15445 19.1761L8.29738 19.8477V22C8.29738 22.4142 8.63317 22.75 9.04738 22.75H14.9527C15.3669 22.75 15.7027 22.4142 15.7027 22V19.8477L16.8456 19.1761L18.6677 20.245C18.8409 20.3465 19.0475 20.3745 19.2413 20.3225C19.4352 20.2706 19.6002 20.1431 19.6993 19.9686L22.652 14.7725C22.8546 14.4158 22.7332 13.9626 22.3794 13.755L20.5486 12.681L20.5486 11.319L22.3795 10.245C22.7333 10.0374 22.8547 9.58417 22.6521 9.22754L19.6994 4.03139C19.6003 3.85689 19.4353 3.72941 19.2414 3.67747C19.0476 3.62553 18.841 3.65346 18.6678 3.75501L16.8456 4.82393L15.7027 4.15233V2C15.7027 1.58579 15.3669 1.25 14.9527 1.25H9.04738ZM11.9999 15.5C13.9329 15.5 15.4999 13.933 15.4999 12C15.4999 10.067 13.9329 8.5 11.9999 8.5C10.0669 8.5 8.49988 10.067 8.49988 12C8.49988 13.933 10.0669 15.5 11.9999 15.5Z"/>
          </svg>
        {/if}
      </div>
    </button>

    <div class="flex flex-col items-center gap-4 mt-auto mb-5">
      <button onclick={onnews} class="size-10 rounded-md flex items-center justify-center"
        onmouseenter={(e) => e.currentTarget.style.background = 'var(--leftbar-btn-hover)'}
        onmouseleave={(e) => e.currentTarget.style.background = ''}
      >
        <svg class="size-6" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="var(--icon-leftbar-unselected)">
          <path fill-rule="evenodd" clip-rule="evenodd" d="M1.25 3C1.25 2.58579 1.58579 2.25 2 2.25H18C18.4142 2.25 18.75 2.58579 18.75 3V7.25H22C22.4142 7.25 22.75 7.58579 22.75 8V19C22.75 20.5188 21.5188 21.75 20 21.75H2C1.58579 21.75 1.25 21.4142 1.25 21V3ZM20 20.25C20.6904 20.25 21.25 19.6904 21.25 19V8.75H18.75V19C18.75 19.6904 19.3096 20.25 20 20.25ZM5.5 8.75H14.5V7.25H5.5V8.75ZM5.5 12.75H14.5V11.25H5.5V12.75ZM5.5 16.75H10V15.25H5.5V16.75Z"/>
        </svg>
      </button>

      <div class="size-3 rounded-full" style="background: var(--uninjected);"></div>
    </div>

  </div>
</div>