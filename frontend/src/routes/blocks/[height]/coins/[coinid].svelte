<script context="module" type="ts">
    import {page} from "$app/stores";
    import { loader } from "@utils/common";
    const {query, load} = loader()
    export let data;
 
    /** @type {import('./__types/[coinid]').RequestHandler} */
    export async function get({ params }) {
    // `params.id` comes from [id].js
    const item = await melscan("/raw/block");
    
    if(item){
        return {
            
        }
    }
    
    return {
        status: 404
    };
    }
</script>

<template>
    <div> {data} </div>
</template>