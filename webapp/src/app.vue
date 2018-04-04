<template>
  <div id="app">
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: 'app'
}
</script>

<style>
:root {
    --purple: #99177d;
}
* {
    padding:0px; margin:0px;
    box-sizing: border-box;
}
a {
  text-decoration: none;
  color: inherit;
}
a:hover { color: var(--purple); }

ul, ol, li {
    list-style: none;
}
</style>
