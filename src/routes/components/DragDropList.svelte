<script lang="ts">
    export let items: string[] = [];
    export let onUpdate: (items: string[]) => void;
    let dragIndex: number | null = null;

    function onDragStart(idx: number) {
        dragIndex = idx;
    }
    function onDragOver(idx: number, event: DragEvent) {
        event.preventDefault();
        if (dragIndex === null || dragIndex === idx) return;
        const updated = [...items];
        const [moved] = updated.splice(dragIndex, 1);
        updated.splice(idx, 0, moved);
        dragIndex = idx;
        if (onUpdate) onUpdate(updated);
    }
    function onDrop() {
        dragIndex = null;
    }
    function remove(idx: number) {
        const updated = items.filter((_, i) => i !== idx);
        if (onUpdate) onUpdate(updated);
    }
</script>

<ul class="dnd-list">
    {#each items as item, idx}
        <li
            class="dnd-item"
            draggable="true"
            on:dragstart={() => onDragStart(idx)}
            on:dragover={(e) => onDragOver(idx, e)}
            on:drop={onDrop}
        >
            <span>{item}</span>
            <button
                type="button"
                class="dnd-remove"
                aria-label="Remove"
                on:click={() => remove(idx)}>&times;</button
            >
        </li>
    {/each}
</ul>

<style>
    .dnd-list {
        list-style: none;
        padding: 0;
        margin: 0.5rem 0 1rem 0;
        border: 1px solid var(--input-border);
        border-radius: 4px;
        background: var(--input-bg);
    }
    .dnd-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.5rem 1rem;
        border-bottom: 1px solid var(--input-border);
        cursor: grab;
        user-select: none;
    }
    .dnd-item:last-child {
        border-bottom: none;
    }
    .dnd-remove {
        background: none;
        border: none;
        color: var(--accent);
        font-size: 1.2rem;
        cursor: pointer;
        margin-left: 0.5rem;
    }
    .dnd-remove:hover {
        color: var(--accent-hover);
    }
</style>
