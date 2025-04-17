<script>
    export let max = 4;
    export let rating = 0;
    export let size = 32;
    export let color = 'gold';
    export let hoverColor = 'lightgoldenrodyellow'; // New hover color

    let hoverRating = 0;

    const handleClick = (value) => {
        rating = rating === value ? 0 : value; // Toggle rating to 0 if the same star is clicked
        dispatch('rate', { rating });
    };

    const handleMouseEnter = (value) => {
        hoverRating = value;
    };

    const handleMouseLeave = () => {
        hoverRating = 0;
    };

    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
</script>

<style>
    .star {
        cursor: pointer;
        user-select: none;
    }
</style>

<div>
    {#each Array(max) as _, i}
        <span
            class="star"
            on:click={() => handleClick(i + 1)}
            on:mouseenter={() => handleMouseEnter(i + 1)}
            on:mouseleave={handleMouseLeave}
            style="font-size: {size}px; color: {i < hoverRating ? hoverColor : (i < rating ? color : 'gray')};"
        >
            {@html i < (hoverRating || rating) ? '★' : '☆'}
        </span>
    {/each}
</div>