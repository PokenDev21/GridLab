<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, createEventDispatcher } from 'svelte';
  import { X, Save, Plus, Trash2 } from '@lucide/svelte';

  const dispatch = createEventDispatcher();
  // Define types for our workspace structures
  type PaneConfig = {
    url: string;
  };

  type WorkspaceConfig = {
    [key: string]: PaneConfig;
  };

  type Workspaces = {
    [key: string]: WorkspaceConfig;
  };

  // Now use proper type definitions
  let workspaces: Workspaces = {};
  let selectedWorkspace: string | null = null;
  let currentConfig: WorkspaceConfig | null = null;
  let newWorkspaceName: string = '';
  let isCreatingNew: boolean = false;

  onMount(async () => {
    await loadWorkspaces();
  });

  async function loadWorkspaces() {
    try {
      workspaces = (await invoke('get_all_workspaces')) as Workspaces;
      console.log('Loaded workspaces:', workspaces);
    } catch (error) {
      console.error('Failed to load workspaces:', error);
    }
  }

  function selectWorkspace(name: string) {
    selectedWorkspace = name;
    currentConfig = JSON.parse(JSON.stringify(workspaces[name]));
  }

  function startNewWorkspace() {
    isCreatingNew = true;
    newWorkspaceName = '';
    currentConfig = {
      main1: { url: 'https://about:blank' },
      main2: { url: 'https://about:blank' },
      main3: { url: 'https://about:blank' },
      main4: { url: 'https://about:blank' },
    };
  }

  // Function to ensure URL has HTTPS protocol
  function ensureHttps(url: string): string {
    if (!url) return '';

    // Remove any existing protocol
    const cleanUrl = url.replace(/^https?:\/\//, '');

    // Special cases that don't need https
    if (
      cleanUrl.startsWith('about:') ||
      cleanUrl.startsWith('file:') ||
      cleanUrl.startsWith('data:')
    ) {
      return url;
    }

    // Add https if it's not already there
    if (!url.startsWith('https://') && !url.startsWith('http://')) {
      return `https://${cleanUrl}`;
    }

    // Convert http to https
    if (url.startsWith('http://')) {
      return url.replace('http://', 'https://');
    }

    return url;
  }

  // Handle URL input changes
  function handleUrlChange(pane: string, value: string) {
    if (currentConfig) {
      currentConfig[pane].url = ensureHttps(value);
    }
  }

  async function saveWorkspace() {
    try {
      const name = isCreatingNew ? newWorkspaceName : selectedWorkspace;
      if (!name) {
        alert('Please enter a workspace name');
        return;
      }

      // Ensure all URLs have HTTPS before saving
      if (currentConfig) {
        Object.keys(currentConfig).forEach((pane) => {
          if (currentConfig && currentConfig[pane]) {
            currentConfig[pane].url = ensureHttps(currentConfig[pane].url);
          }
        });
      }

      await invoke('save_workspace', { name, config: currentConfig });
      console.log(`Saved workspace: ${name}`);

      // Reset and reload
      isCreatingNew = false;
      await loadWorkspaces();
      selectWorkspace(name);

      dispatch('workspacesChanged');
    } catch (error) {
      console.error('Failed to save workspace:', error);
    }
  }

  async function deleteWorkspace() {
    if (!selectedWorkspace) return;

    if (confirm(`Are you sure you want to delete "${selectedWorkspace}"?`)) {
      try {
        await invoke('delete_workspace', { name: selectedWorkspace });
        console.log(`Deleted workspace: ${selectedWorkspace}`);

        // Reset and reload
        selectedWorkspace = null;
        currentConfig = null;

        await loadWorkspaces();
        dispatch('workspacesChanged');
      } catch (error) {
        console.error('Failed to delete workspace:', error);
      }
    }
  }
</script>

<div class="h-full w-full overflow-auto p-6">
  <div class="mb-8 flex items-center justify-between">
    <h1 class="text-3xl font-bold">Workspace Settings</h1>
  </div>

  <div class="flex h-[calc(100%-4rem)]">
    <!-- Left sidebar - workspace list -->
    <div class="w-1/4 border-r pr-4">
      <div class="mb-4 flex items-center justify-between">
        <h2 class="text-xl font-semibold">Workspaces</h2>
        <button
          class="rounded-full bg-gray-900 p-2 text-white hover:bg-gray-700"
          on:click={startNewWorkspace}
        >
          <Plus size={16} />
        </button>
      </div>

      <div class="space-y-2">
        {#each Object.keys(workspaces) as name}
          <button
            class="w-full rounded p-2 text-left hover:bg-gray-100 {selectedWorkspace === name
              ? 'bg-gray-200'
              : ''}"
            on:click={() => selectWorkspace(name)}
          >
            {name}
          </button>
        {/each}
      </div>
    </div>

    <!-- Right content - workspace editor -->
    <div class="w-3/4 pl-6">
      {#if isCreatingNew}
        <div class="mb-6">
          <label for="workspace-name" class="mb-1 block text-sm font-medium">Workspace Name:</label>
          <input
            id="workspace-name"
            class="w-full rounded border p-2"
            bind:value={newWorkspaceName}
            placeholder="Enter workspace name"
          />
        </div>
      {:else if selectedWorkspace}
        <h2 class="mb-6 text-2xl font-semibold">{selectedWorkspace}</h2>
      {:else}
        <div class="flex h-full items-center justify-center text-gray-500">
          <p>Select a workspace or create a new one</p>
        </div>
      {/if}

      {#if currentConfig}
        <div class="space-y-6">
          {#each ['main1', 'main2', 'main3', 'main4'] as pane}
            <div class="rounded-lg border p-4">
              <h3 class="mb-3 text-lg font-medium">Pane {pane.replace('main', '')}</h3>
              <div>
                <input
                  id={`url-${pane}`}
                  class="w-full rounded border p-2"
                  value={currentConfig[pane].url}
                  on:input={(e) => {
                    const target = e.target as HTMLInputElement | null;
                    if (target) handleUrlChange(pane, target.value);
                  }}
                  placeholder="example.com (https:// will be added automatically)"
                />
              </div>
            </div>
          {/each}

          <div class="mb-auto flex justify-between pt-4">
            <button
              class="flex items-center rounded bg-red-500 px-4 py-2 text-white hover:bg-red-600"
              on:click={deleteWorkspace}
              disabled={isCreatingNew}
            >
              <Trash2 size={18} class="mr-2" />
              Delete
            </button>

            <button
              class="flex items-center rounded bg-green-500 px-4 py-2 text-white hover:bg-green-600"
              on:click={saveWorkspace}
            >
              <Save size={18} class="mr-2" />
              Save
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>
