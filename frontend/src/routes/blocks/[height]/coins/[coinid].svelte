<script context="module" lang="ts">
    import {page} from "$app/stores";
    import { loader } from "@utils/common";
    const {refresh, load} = loader()
    export let data;
 
</script>
<script lang="ts">

</script>
<template>
    <div> {data} </div>
</template>