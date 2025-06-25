<script lang="ts">
  import { LayoutDashboard, Plus, Settings as SettingsIcon } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher, onMount } from 'svelte';

  export let isExpanded = false;
  export let CreateWorkspacefunc: (name: string) => void;

  // Track workspaces
  type Workspaces = {
    [key: string]: any;
  };

  let workspaces: Workspaces = {};
  let isLoading = true;

  $: updateSidebarWidth(isExpanded ? 288 : 64);
  const dispatch = createEventDispatcher();

  onMount(async () => {
    await loadWorkspaces();
  });

  async function loadWorkspaces() {
    isLoading = true;
    try {
      workspaces = (await invoke('get_all_workspaces')) as Workspaces;
      console.log('Loaded workspaces:', workspaces);
    } catch (error) {
      console.error('Failed to load workspaces:', error);
    } finally {
      isLoading = false;
    }
  }
  export function refreshWorkspaces() {
    loadWorkspaces();
  }
  async function updateSidebarWidth(width: number) {
    try {
      await invoke('update_sidebar_width', { width });
      console.log(`Updated sidebar width to: ${width}px`);
    } catch (error) {
      console.error('Failed to update sidebar width:', error);
    }
  }

  async function setFullscreen(fullscreenValue: boolean) {
    dispatch('settingsChange', fullscreenValue);
    try {
      await invoke('toggle_fullscreen', { fullscreen: fullscreenValue });
      console.log(`Toggled fullscreen: ${fullscreenValue}`);
    } catch (error) {
      console.error('Failed to toggle fullscreen:', error);
    }
  }
</script>

<div
  class="flex h-full flex-col items-center bg-neutral-50 p-3 shadow-xl transition-all duration-0 ease-in"
  class:w-72={isExpanded}
  class:w-16={!isExpanded}
  style="box-shadow: inset 0 0 0 0.2px black;"
>
  <!-- Workspace list -->
  {#if isLoading}
    <div class="my-4 text-center text-sm text-gray-500">Loading...</div>
  {:else if Object.keys(workspaces).length === 0}
    <div class="my-4 text-center text-sm text-gray-500">No workspaces yet</div>
  {:else}
    {#each Object.keys(workspaces) as workspace}
      <button
        class="flex h-10 w-full items-center rounded hover:bg-gray-100"
        on:click={() => {
          CreateWorkspacefunc(workspace);
          setFullscreen(false);
        }}
      >
        <LayoutDashboard size="24" class={isExpanded ? 'mx-4' : 'mx-auto'} />
        <div class="overflow-hidden {isExpanded ? 'w-auto' : 'w-0'}">
          <div class="whitespace-nowrap text-[1rem]">{workspace}</div>
        </div>
      </button>
    {/each}
  {/if}

  <!-- Settings button -->
  <button
    class="mt-4 flex h-10 w-full items-center rounded hover:bg-gray-100"
    on:click={() => setFullscreen(true)}
  >
    <SettingsIcon size="24" class={isExpanded ? 'mx-4' : 'mx-auto'} />
    <div class="overflow-hidden {isExpanded ? 'w-auto' : 'w-0'}">
      <div class="whitespace-nowrap text-[1rem]">Settings</div>
    </div>
  </button>

  <div class="mb-2 mt-auto flex w-full flex-col items-center">
    <div class="h-[0.15rem] w-full bg-black"></div>
  </div>
</div>
