<template>
  <div id="app">
    <mnav id="mnav"></mnav>
    <router-view></router-view>
  </div>
</template>

<style>
 * {
   padding:0px; margin:0px;
   box-sizing: border-box;
 }
 a {
   text-decoration: none;
   color: inherit;
 }
 a:hover { color: var(--purple); }
</style>

<script>
 import Mnav from '@/components/nav/Mnav'
 export default {
   name: 'app',
   components: {
     "mnav": Mnav
   },
 }
</script>
