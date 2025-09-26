<script lang="ts">
    import DragDropList from "../components/DragDropList.svelte";
    import { goto } from "$app/navigation";
    import { core } from "@tauri-apps/api";
    import { open } from '@tauri-apps/plugin-dialog';
    import { onMount } from "svelte";

    let name = "";
    let transformerList: string[] = [];
    const transformerOptions = [
        { key: "custom_py", label: "Custom (Python)" },
        { key: "pretty_json", label: "Pretty JSON" },
        { key: "json_unescape", label: "JSON Unescape" },
    ];
    let selection = transformerOptions[0].key;
    // Stores absolute path to the python script
    let scriptPath = "";

    onMount(() => {
        // Restore last selected script from localStorage if available
        try {
            const stored = localStorage.getItem("py_script");
            if (stored) scriptPath = stored;
        } catch (e) {
            // ignore
        }
    });

    async function selectPythonScript() {
        try {
            const res: any = await open({
                multiple: false,
                filters: [
                    { name: "Python", extensions: ["py"] },
                ],
                title: "Select Python script",
            });

            let selectedPath: string | null = null;
            if (Array.isArray(res)) {
                if (res.length > 0 && typeof res[0] === "string") selectedPath = res[0];
            } else if (res && typeof res === "string") {
                selectedPath = res;
            }

            if (selectedPath) {
                scriptPath = selectedPath;
                try {
                    localStorage.setItem("py_script", scriptPath);
                } catch (e) {
                    // ignore
                }
            }
        } catch (err) {
            console.error("Failed to select file", err);
        }
    }

    async function save() {
        if (name && transformerList.length > 0) {
            let extraArgs = undefined;
            if (selection === "custom_py") {
                extraArgs = { py_script: scriptPath };
            }
            await core.invoke("add_preset", {
                name: name,
                transformerList: transformerList,
                extraArgs: extraArgs,
            });
            goto("/");
        }
    }
    function cancel() {
        goto("/");
    }
    function addSelectionToList() {
        if (selection && !transformerList.includes(selection)) {
            transformerList = [...transformerList, selection];
        }
    }
    function updateDndItems(items: string[]) {
        transformerList = items;
    }
</script>

<main class="add-preset-page">
    <h2>Add New Preset</h2>
    <label>
        Name:
        <input type="text" bind:value={name} placeholder="Preset name" />
    </label>
    <label>
        Pick a transformer:
        <div class="select-row">
            <select bind:value={selection}>
                {#each transformerOptions as option}
                    <option value={option.key}>{option.label}</option>
                {/each}
            </select>
            <button
                type="button"
                class="add-selection-btn"
                on:click={addSelectionToList}
                aria-label="Add to list">+</button
            >
        </div>
    </label>
    {#if selection === "custom_py"}
        <label>
            Python Script:
            <div style="display:flex;gap:0.5rem;align-items:center;">
                <input
                    type="text"
                    bind:value={scriptPath}
                    placeholder="Select a python script file"
                    readonly
                    style="flex:1"
                />
                <button type="button" class="select-file-btn" on:click={selectPythonScript}>Select file</button>
            </div>
        </label>
    {/if}
    <label>
        Transformer(s):
        <DragDropList items={transformerList} onUpdate={updateDndItems} />
    </label>
    <div class="actions">
        <button
            type="button"
            class="save-btn"
            on:click={save}
            disabled={!name || transformerList.length === 0}>Save</button
        >
        <button type="button" class="cancel-btn" on:click={cancel}
            >Cancel</button
        >
    </div>
</main>

<style>
    .add-preset-page {
        max-width: 480px;
        margin: 2rem auto;
        background: var(--background);
        color: var(--text);
        border-radius: 10px;
        box-shadow: 0 4px 32px rgba(0, 0, 0, 0.08);
        padding: 2rem 2.5rem;
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }
    .add-preset-page h2 {
        margin: 0 0 0.5rem 0;
        font-size: 1.4rem;
        color: var(--accent);
    }
    .add-preset-page label {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        font-size: 1rem;
    }
    .add-preset-page input[type="text"],
    .add-preset-page select {
        font-size: 1rem;
        padding: 0.5rem;
        border: 1px solid var(--input-border);
        border-radius: 4px;
        background: var(--input-bg);
        color: var(--text);
        margin-top: 0.25rem;
    }
    .actions {
        display: flex;
        gap: 0.75rem;
        justify-content: flex-end;
        margin-top: 0.5rem;
    }
    .save-btn {
        background: var(--accent);
        color: var(--text);
        border: none;
        border-radius: 4px;
        padding: 0.5rem 1.25rem;
        font-size: 1rem;
        cursor: pointer;
        transition: background 0.2s;
    }
    .save-btn:disabled {
        background: var(--sidebar-border);
        color: #aaa;
        cursor: not-allowed;
    }
    .save-btn:hover:enabled {
        background: var(--accent-hover);
    }
    .cancel-btn {
        background: none;
        color: var(--accent);
        border: 1px solid var(--accent);
        border-radius: 4px;
        padding: 0.5rem 1.25rem;
        font-size: 1rem;
        cursor: pointer;
        transition:
            background 0.2s,
            color 0.2s;
    }
    .cancel-btn:hover {
        background: var(--accent);
        color: var(--background);
    }
    .select-row {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.5rem;
    }
    .select-row select {
        width: 100%;
        min-width: 120px;
        max-width: 220px;
        flex: 1 1 auto;
        padding-right: 2rem;
        box-sizing: border-box;
        background-color: var(--background) !important;
        color: var(--text) !important;
        border: 1px solid var(--accent);
        appearance: none;
    }
    .add-selection-btn {
        background: var(--accent);
        color: var(--background);
        border: none;
        border-radius: 50%;
        width: 32px;
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 1.3rem;
        font-weight: bold;
        box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
        cursor: pointer;
        transition:
            background 0.2s,
            color 0.2s;
        margin-left: 0.25rem;
        padding: 0;
    }
    .add-selection-btn:hover {
        background: var(--accent-hover);
        color: var(--text);
    }
    .select-file-btn {
        background: var(--input-bg);
        color: var(--text);
        border: 1px solid var(--input-border);
        border-radius: 4px;
        padding: 0.45rem 0.75rem;
        cursor: pointer;
        transition: background 0.15s, color 0.15s, border-color 0.15s;
    }
    .select-file-btn:hover {
        background: var(--input-bg-hover, var(--input-bg));
        border-color: var(--accent);
    }
</style>
