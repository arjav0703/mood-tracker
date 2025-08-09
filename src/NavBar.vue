<template>  <div class="navbar">
    <button @click="selectTab('Home')">Home</button>
    <button @click="selectTab('Graphs')">About</button>
    <button @click="selectTab('contact')">Contact</button>
    </div>
</template>
<script>
export default {
  data() {
    return {
      currentTab: 'home',
    };
  },
  methods: {
    selectTab(tab) {
      this.currentTab = tab;
      this.$emit('tab-selected', tab);
    },
  },
};
</script>
<style>
.navbar {
  position: fixed;
  bottom: 4.75%;
  width: 150%;
  display: flex;
  justify-content: space-around;
  background-color: #fff;
  border-top: 1px solid #ccc;
}
</style>
