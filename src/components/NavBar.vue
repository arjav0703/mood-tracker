<template>
    <div class="navbar">
        <button
            @click="selectTab('Home')"
            :class="{ active: currentTab === 'Home' }"
            class="nav-button"
        >
            <span class="nav-icon">🏠</span>
        </button>
        <button
            @click="selectTab('Graphs')"
            :class="{ active: currentTab === 'Graphs' }"
            class="nav-button"
        >
            <span class="nav-icon">📊</span>
        </button>
    </div>
</template>

<script>
export default {
    props: {
        currentTab: {
            type: String,
            default: "Home",
        },
    },
    methods: {
        selectTab(tab) {
            this.$emit("tab-selected", tab);
        },
    },
};
</script>

<style scoped>
.navbar {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(20px);
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    justify-content: space-around;
    padding: 12px 20px 24px;
    z-index: 100;
    box-shadow: 0 -4px 20px rgba(0, 0, 0, 0.1);
}

.nav-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    padding: 12px;
    cursor: pointer;
    border-radius: 12px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    min-width: 60px;
    min-height: 60px;
    font-size: inherit;
    color: #7f8c8d;
}

.nav-button:hover {
    background: rgba(74, 144, 226, 0.1);
    transform: translateY(-2px);
}

.nav-button.active {
    color: #4a90e2;
    background: rgba(74, 144, 226, 0.15);
}

.nav-button.active .nav-icon {
    transform: scale(1.1);
}

.nav-icon {
    font-size: 1.8rem;
    transition: transform 0.3s ease;
}

@media (max-width: 480px) {
    .navbar {
        padding: 10px 15px 20px;
    }

    .nav-button {
        min-width: 50px;
        min-height: 50px;
        padding: 10px;
    }

    .nav-icon {
        font-size: 1.5rem;
    }
}

@media (prefers-color-scheme: dark) {
    .navbar {
        background: rgba(44, 62, 80, 0.95);
        border-top-color: rgba(255, 255, 255, 0.1);
    }

    .nav-button {
        color: #bdc3c7;
    }

    .nav-button:hover {
        background: rgba(74, 144, 226, 0.2);
    }

    .nav-button.active {
        color: #74b9ff;
        background: rgba(74, 144, 226, 0.25);
    }
}
</style>
