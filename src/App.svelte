<script lang="ts">
  import Sidebar from './lib/Sidebar.svelte';
  import { writable } from 'svelte/store';
  const isExpanded = writable(false);
  let blur = '6rem';
  function openWorkspace() {}
</script>

<div class="relative flex h-full w-full overflow-hidden">
  <div
    class="fixed left-0 top-0 z-50 h-full"
    role="navigation"
    on:mouseenter={() => ($isExpanded = true)}
    on:mouseleave={() => ($isExpanded = false)}
  >
    <Sidebar isExpanded={$isExpanded} CreateWorkspacefunc={openWorkspace} />
  </div>
  <div
    class="absolute inset-0 -z-10 opacity-50"
    style="background-image: url('/background.webp'); background-size: cover; background-position: center; filter: blur(100px);"
  ></div>
  <div
    class="flex h-screen w-full transition-all duration-300 ease-in-out"
    class:ml-72={$isExpanded}
    class:ml-18={!$isExpanded}
  >
    <div class="mx-auto flex h-screen w-3/4 flex-col items-center justify-center text-center">
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
      <div class="z-10">
        <h1 class="text-5xl font-bold text-white">Grid Lab</h1>
        <h2 class="mt-3 text-xl text-white">Select an environment from the left to begin</h2>
        <button class="w-30 h-10 p-4">Call Rust function</button>
      </div>
    </div>
  </div>
</div>
