<script>
import { createEventDispatcher } from 'svelte';
const dispatch = createEventDispatcher();
export let buttonText = 'Click me';
export let buttonID = 'btn_default';
</script>



<style>
button {
    padding: 10px 20px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}
button:active {
  box-shadow: 2px 2px 5px #000000;
}
</style>
  


<button on:click={() => dispatch('click')} id = {buttonID}>
    <slot>{buttonText}</slot>
</button>