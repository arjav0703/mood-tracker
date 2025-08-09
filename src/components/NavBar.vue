<template>  <div class="navbar">
    <button @click="selectTab('Home')">🏠</button>
    <button @click="selectTab('Graphs')">📊</button>
    <button @click="selectTab('contact')"> 📞</button>
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
  bottom: 7.5%;
  width: 96.5%;
  display: flex;
  justify-content: space-around; 
  font-size: larger;
  }
</style>