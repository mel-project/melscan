<script lang="ts">
    import type { BreadCrumb } from "@utils/page-types";


    export let breadcrumbs: BreadCrumb[]
</script>

<template>
    <div class="breadcrumbs">
        {#each breadcrumbs as breadcrumb,index}
            <a class="breadcrumb" href={breadcrumb.href}>{breadcrumb.title}</a>
            {#if index + 1 !== breadcrumbs.length}🢒
            {/if}
        {/each}
    </div>
</template>

<style lang="scss">
    .breadcrumbs{
    }
    .breadcrumb{
        color: gray;
        margin: 0 .1em 0 .3em;
    }
    .breadcrumb:last-child{
        color: black;
    }
</style>