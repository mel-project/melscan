

<script lang="ts">
    
</script>

<template>
    <div class="m-3">
        <table class="table-fixed w-full text-sm text-left">
            <tbody>
                <tr>
                    <td class="text-black text-opacity-50 font-bold w-1/3">Height</td>
                    <td><a href="/blocks/{height}" class="text-blue-600 font-medium">{height}</a></td>
                </tr>
                <tr>
                    <td class="text-black text-opacity-50 font-bold">
                        <span class="name">Kind</span>
                        {tooltips['kind']}
                    </td>
                    <td class="font-medium">{kind}</td>
                </tr>
                <tr>
                    <td class="text-black text-opacity-50 font-bold">
                        <span class="name">Hash</span>
                    </td>
                    <td class="font-medium">{txhash}</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>