<script lang="ts">
  import Sidebar from './lib/Sidebar.svelte';
  import type SidebarType from './lib/Sidebar.svelte';
  import Settings from './lib/Settings.svelte';
  import { writable } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  const isExpanded = writable(false);
  const isSettings = writable(false);
  let blur = '6rem';
  let sidebarComponent: SidebarType | null = null;
  $: sidebarWidth = $isExpanded ? 288 : 64;
  async function openWorkspace(layoutName: string) {
    try {
      await invoke('load_workspace', { layoutName });
      console.log(`Loaded workspace: ${layoutName}`);
    } catch (error) {
      console.error('Failed to load workspace:', error);
    }
  }
  function handleWorkspacesChanged() {
    if (sidebarComponent) {
      sidebarComponent.refreshWorkspaces();
    }
  }
</script>

<div class="relative flex h-full w-full overflow-hidden">
  <div
    class="fixed left-0 top-0 z-50 h-full"
    role="navigation"
    on:mouseenter={() => ($isExpanded = true)}
    on:mouseleave={() => ($isExpanded = false)}
  >
    <Sidebar
      bind:this={sidebarComponent}
      isExpanded={$isExpanded}
      CreateWorkspacefunc={openWorkspace}
      on:settingsChange={(e) => ($isSettings = e.detail)}
    />
  </div>
  <div
    class="absolute inset-0 -z-10 opacity-50"
    style="background-image: url('/background.webp'); background-size: cover; background-position: center; filter: blur(100px);"
  ></div>
  <div
    class="flex h-screen w-full transition-all duration-0 ease-in"
    style="margin-left: {sidebarWidth}px"
  >
    <div class="mx-auto flex h-screen w-full flex-col items-center justify-center text-center">
      <div
        class="absolute -bottom-[30%] z-0 mx-auto h-1/2 w-1/2 animate-blob rounded-full bg-fuchsia-500 opacity-40 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 1s;"
      ></div>
      <div
        class="absolute -bottom-[-10%] z-0 mx-auto h-1/2 w-1/2 rounded-full bg-cyan-100 opacity-60 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 1s;"
      ></div>
      <div
        class="absolute -top-[10%] right-[-10%] z-0 h-1/2 w-1/2 animate-blob rounded-full bg-rose-300 opacity-50 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 2s;"
      ></div>
      <div
        class="absolute -right-[10%] bottom-0 z-0 h-1/2 w-1/2 animate-blob rounded-full bg-orange-400 opacity-50 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 3s;"
      ></div>
      <div
        class="absolute left-[30%] top-0 z-0 h-1/2 w-1/2 animate-blob rounded-full bg-red-400 opacity-30 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 0.5s;"
      ></div>
      <div
        class="absolute left-[0%] z-0 my-auto h-3/4 w-1/2 animate-blob rounded-full bg-purple-400 opacity-30 mix-blend-multiply"
        style="filter: blur({blur}); animation-delay: 0.5s;"
      ></div>
      {#if $isSettings}
        <div class="z-10 flex h-full w-full items-center justify-center">
          <div
            class="m-10 rounded-lg bg-neutral-50 opacity-70 shadow-lg backdrop-blur-lg"
            style="width: calc(100%); height: calc(100% - 3rem);"
          >
            <Settings on:workspacesChanged={handleWorkspacesChanged} />
          </div>
        </div>
      {:else}
        <div class="z-10 w-3/4">
          <h1 class="text-5xl font-bold text-white">Grid Lab</h1>
          <h2 class="mt-3 text-xl text-white">Select an environment from the left to begin</h2>
        </div>
      {/if}
    </div>
  </div>
</div>
