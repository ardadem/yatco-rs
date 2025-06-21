<script lang="ts">
  import { core } from "@tauri-apps/api";
  import { goto } from "$app/navigation";
  import "../app.css";
  import { onMount, onDestroy } from "svelte";

  let presets: string[] = [];
  let selectedPreset = "";

  let inputText = "";
  let outputText = "";

  let contextMenuVisible = false;
  let contextMenuX = 0;
  let contextMenuY = 0;
  let contextMenuPreset = "";

  let contextMenuRef: HTMLDivElement | null = null;
  let outsideClickHandler: ((event: MouseEvent) => void) | null = null;

  onMount(async () => {
    const rawPresets = (await core.invoke("get_presets")) as [
      string,
      string[],
    ][];
    presets = rawPresets.map(([name]) => name);
    if (presets && presets.length > 0) {
      selectedPreset = presets[0];
    }
  });

  function selectPreset(presetName: string) {
    selectedPreset = presetName;
    inputText = "";
    outputText = "";
  }

  function handleInput() {
    if (selectedPreset === null) {
      return;
    }

    core
      .invoke("transform", {
        input: inputText,
        presetName: selectedPreset,
      })
      .then((result) => {
        outputText = result as string;
      });
  }

  function openAddModal() {
    goto("/add-preset");
  }

  function showContextMenu(event: MouseEvent, presetName: string) {
    event.preventDefault();
    contextMenuVisible = true;
    contextMenuX = event.clientX;
    contextMenuY = event.clientY;
    contextMenuPreset = presetName;
    // Add global click listener
    setTimeout(() => {
      outsideClickHandler = (e: MouseEvent) => {
        if (contextMenuRef && !contextMenuRef.contains(e.target as Node)) {
          hideContextMenu();
        }
      };
      document.addEventListener("mousedown", outsideClickHandler);
    });
  }

  function hideContextMenu() {
    contextMenuVisible = false;
    contextMenuPreset = "";
    // Remove global click listener
    if (outsideClickHandler) {
      document.removeEventListener("mousedown", outsideClickHandler);
      outsideClickHandler = null;
    }
  }

  onDestroy(() => {
    if (outsideClickHandler) {
      document.removeEventListener("mousedown", outsideClickHandler);
    }
  });

  async function deletePreset(presetName: string) {
    await core.invoke("delete_preset", { name: presetName });
    presets = presets.filter((p) => p !== presetName);
    if (selectedPreset === presetName) {
      selectedPreset = presets.length > 0 ? presets[0] : "";
    }
    hideContextMenu();
  }
</script>

<div class="layout">
  <aside class="sidebar">
    <div class="sidebar-header">
      <h2>Presets</h2>
      <button
        type="button"
        class="add-btn"
        title="Add Preset"
        on:click={openAddModal}>+</button
      >
    </div>
    <ul>
      {#each presets as presetName}
        <li
          class:selected={selectedPreset === presetName}
          on:contextmenu={(e) => showContextMenu(e, presetName)}
        >
          <button
            type="button"
            on:click={() => selectPreset(presetName)}
            class="preset-btn"
          >
            {presetName}
          </button>
        </li>
      {/each}
    </ul>

    {#if contextMenuVisible}
      <div
        bind:this={contextMenuRef}
        class="context-menu"
        style="top: {contextMenuY}px; left: {contextMenuX}px;"
        role="menu"
        tabindex="0"
        aria-label="Preset options"
        on:keydown={(e) => {
          if (e.key === "Escape") hideContextMenu();
        }}
      >
        <button
          class="context-menu-delete"
          role="menuitem"
          on:click={() => deletePreset(contextMenuPreset)}
        >
          Delete
        </button>
      </div>
    {/if}
  </aside>
  <main class="main-content">
    <div class="inputs">
      <textarea
        class="input-area"
        bind:value={inputText}
        placeholder="Enter text here..."
      ></textarea>
      <textarea
        class="output-area"
        value={outputText}
        readonly
        placeholder="Output will appear here..."
      ></textarea>
    </div>
  </main>
</div>

<!-- Floating Action Button (FAB) -->
<button type="button" on:click={() => handleInput()} class="fab">
  Transform
</button>

<style>
  .layout {
    display: flex;
    height: 100vh;
    background: var(--background);
    color: var(--text);
  }
  .sidebar {
    width: 220px;
    background: var(--sidebar-bg);
    padding: 1rem;
    border-right: 1px solid var(--sidebar-border);
    display: flex;
    flex-direction: column;
  }
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
    gap: 0;
  }
  .sidebar-header h2 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--accent);
    letter-spacing: 1px;
    font-weight: 600;
    line-height: 1;
  }
  .sidebar ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .sidebar li {
    padding: 0;
    margin-bottom: 0.25rem;
  }
  .preset-btn {
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    text-align: left;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    color: var(--text);
    transition: background 0.15s;
  }
  .sidebar li.selected .preset-btn {
    background: var(--sidebar-selected);
    font-weight: bold;
    color: var(--accent);
  }
  .preset-btn:hover {
    background: var(--sidebar-hover);
    color: var(--accent);
  }
  .main-content {
    flex: 1;
    display: flex;
    align-items: stretch;
    justify-content: stretch;
    background: var(--background);
    padding: 0;
  }
  .inputs {
    display: flex;
    gap: 0;
    width: 100%;
    height: 100%;
  }
  .input-area,
  .output-area {
    width: 50%;
    height: 100vh;
    font-size: 1rem;
    padding: 1rem;
    border: none;
    border-radius: 0;
    resize: none;
    background: var(--input-bg);
    color: var(--text);
    box-sizing: border-box;
    outline: none;
  }
  .input-area {
    border-right: 1px solid var(--input-border);
  }
  .output-area {
    background: var(--output-bg);
    border-left: 1px solid var(--output-border);
  }
  ::-webkit-scrollbar {
    width: 8px;
    background: var(--sidebar-bg);
  }
  ::-webkit-scrollbar-thumb {
    background: var(--sidebar-border);
    border-radius: 4px;
  }
  .fab {
    position: fixed;
    right: 2rem;
    bottom: 2rem;
    min-width: 56px;
    height: 56px;
    padding: 0 1.5rem;
    border-radius: 28px;
    background: var(--accent);
    color: var(--text);
    border: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    font-size: 1.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 1000;
    transition:
      background 0.2s,
      box-shadow 0.2s;
    white-space: nowrap;
  }
  .fab:hover {
    background: var(--accent-hover);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
  }
  .add-btn {
    margin: 0;
    background: var(--accent);
    color: var(--text);
    border: none;
    border-radius: 50%;
    width: 28px;
    height: 28px;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.2s;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
  }
  .add-btn:hover {
    background: var(--accent-hover);
  }
  .context-menu {
    position: fixed;
    z-index: 2000;
    background: var(--input-bg);
    border: 1px solid var(--sidebar-border);
    border-radius: 6px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    padding: 0.5rem 0.75rem;
    min-width: 120px;
    display: flex;
    flex-direction: column;
  }
  .context-menu-delete {
    background: none;
    border: none;
    color: var(--accent);
    font-size: 1rem;
    padding: 0.5rem 0;
    text-align: left;
    cursor: pointer;
  }
  .context-menu-delete:hover {
    color: var(--accent-hover);
    background: var(--sidebar-hover);
  }
</style>
