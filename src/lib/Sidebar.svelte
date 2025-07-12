<script lang="ts">
  import { LayoutDashboard, Settings as SettingsIcon, House, SquarePen } from '@lucide/svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher, onMount } from 'svelte';

  export let isExpanded = false;
  export let CreateWorkspacefunc: (name: string) => void;

  type Workspaces = {
    [key: string]: any;
  };

  let workspaces: Workspaces = {};
  let isLoading = true;

  const dispatch = createEventDispatcher();

  // Update sidebar width when `isExpanded` changes
  $: if (isExpanded !== undefined) {
    updateSidebarWidth(isExpanded ? 288 : 64);
  }

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

  async function setFullscreen(fullscreenValue: boolean, settingsValue: boolean) {
    dispatch('settingsChange', settingsValue);
    try {
      await invoke('toggle_fullscreen', { fullscreen: fullscreenValue });
      console.log(`Toggled fullscreen: ${fullscreenValue}`);
    } catch (error) {
      console.error('Failed to toggle fullscreen:', error);
    }
  }
</script>

<div
  class="transition-width flex h-full flex-col items-center bg-neutral-50 p-3 shadow-xl duration-200 ease-in-out"
  style="width: {isExpanded ? 288 : 64}px; box-shadow: inset 0 0 0 0.2px black;"
>
  <!-- Optional top Settings button that reloads the app -->
  <button
    class="mt-4 flex h-10 w-full items-center rounded hover:bg-gray-100"
    on:click={() => {
      setFullscreen(true, false);
    }}
  >
    <House class={`h-6 w-6 ${isExpanded ? 'mx-4' : 'mx-auto'}`} />
    <div class={`overflow-hidden ${isExpanded ? 'w-auto' : 'w-0'}`}>
      <div class="whitespace-nowrap text-[1rem]">Home</div>
    </div>
  </button>

  <!-- Workspace List -->
  {#if isLoading}
    <div class="my-4 text-center text-sm text-gray-500">Loading...</div>
  {:else if Object.keys(workspaces).length === 0}
    <div class="my-4 text-center text-sm text-gray-500">No workspaces yet</div>
  {:else}
    {#each Object.keys(workspaces) as workspace (workspace)}
      <button
        class="flex h-10 w-full items-center rounded hover:bg-gray-100"
        on:click={() => {
          CreateWorkspacefunc(workspace);
          setFullscreen(false, false);
        }}
      >
        <LayoutDashboard size="24" class={isExpanded ? 'mx-4' : 'mx-auto'} />
        <div class={`overflow-hidden ${isExpanded ? 'w-auto' : 'w-0'}`}>
          <div class="whitespace-nowrap text-[1rem]">{workspace}</div>
        </div>
      </button>
    {/each}
  {/if}

  <button
    class="mt-4 flex h-10 w-full items-center rounded hover:bg-gray-100"
    on:click={() => setFullscreen(true, true)}
  >
    <SquarePen class={`h-6 w-6 ${isExpanded ? 'mx-4' : 'mx-auto'}`} />
    <div class={`overflow-hidden ${isExpanded ? 'w-auto' : 'w-0'}`}>
      <div class="whitespace-nowrap text-[1rem]">Settings</div>
    </div>
  </button>

  <div class="mb-2 mt-auto flex w-full flex-col items-center">
    <div class="h-[0.15rem] w-full bg-black"></div>
  </div>
</div>
