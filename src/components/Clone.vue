<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
    (e: "clone", data: { repositoryUrl: string; localPath: string }): void;
}>();

const repositoryUrl = ref("");
const localPath = ref("/Users/username/Projects");
const showAdvancedOptions = ref(false);

const handleSubmit = () => {
    emit("clone", {
        repositoryUrl: repositoryUrl.value,
        localPath: localPath.value,
    });
};

const handleBrowse = () => {
    // TODO: ディレクトリ選択ダイアログの実装
    console.log("Browse directory");
};
</script>

<template>
    <div class="clone-container">
        <div class="clone-card">
            <div class="clone-header">
                <div class="logo">
                    <svg
                        width="32"
                        height="32"
                        viewBox="0 0 24 24"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.58 20 4 16.42 4 12C4 7.58 7.58 4 12 4C16.42 4 20 7.58 20 12C20 16.42 16.42 20 12 20Z"
                            fill="#10B981"
                        />
                    </svg>
                </div>
                <h1 class="clone-title">Clone Repository</h1>
                <p class="clone-subtitle">
                    Enter the repository URL to get started
                </p>
            </div>

            <form class="clone-form" @submit.prevent="handleSubmit">
                <div class="form-group">
                    <label for="repository-url">Repository URL</label>
                    <div class="input-wrapper">
                        <svg
                            width="20"
                            height="20"
                            viewBox="0 0 20 20"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M15 2.5H5C3.61929 2.5 2.5 3.61929 2.5 5V15C2.5 16.3807 3.61929 17.5 5 17.5H15C16.3807 17.5 17.5 16.3807 17.5 15V5C17.5 3.61929 16.3807 2.5 15 2.5Z"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                            <path
                                d="M7.5 10L12.5 10"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        <input
                            id="repository-url"
                            v-model="repositoryUrl"
                            type="text"
                            placeholder="https://github.com/username/repository.git"
                            required
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label for="local-path">Local Destination</label>
                    <div class="input-group">
                        <div class="input-wrapper">
                            <svg
                                width="20"
                                height="20"
                                viewBox="0 0 20 20"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M2.5 5C2.5 4.17157 3.17157 3.5 4 3.5H16C16.8284 3.5 17.5 4.17157 17.5 5V15C17.5 15.8284 16.8284 16.5 16 16.5H4C3.17157 16.5 2.5 15.8284 2.5 15V5Z"
                                    stroke="currentColor"
                                    stroke-width="1.5"
                                />
                            </svg>
                            <input
                                id="local-path"
                                v-model="localPath"
                                type="text"
                                placeholder="/Users/username/Projects"
                                required
                            />
                        </div>
                        <button
                            type="button"
                            class="btn btn-secondary browse-button"
                            @click="handleBrowse"
                        >
                            <svg
                                width="20"
                                height="20"
                                viewBox="0 0 20 20"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    d="M17.5 12.5V15C17.5 15.8284 16.8284 16.5 16 16.5H4C3.17157 16.5 2.5 15.8284 2.5 15V5C2.5 4.17157 3.17157 3.5 4 3.5H7.5"
                                    stroke="currentColor"
                                    stroke-width="1.5"
                                    stroke-linecap="round"
                                />
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="advanced-options">
                    <button
                        type="button"
                        class="advanced-toggle"
                        @click="showAdvancedOptions = !showAdvancedOptions"
                    >
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                            :class="{ rotate: showAdvancedOptions }"
                        >
                            <path
                                d="M4 6L8 10L12 6"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        Advanced Options
                    </button>
                </div>

                <button type="submit" class="clone-button">Clone</button>
            </form>
        </div>
    </div>
</template>

<style scoped>
.clone-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-background);
    padding: 1rem;
}

.clone-card {
    background-color: var(--color-surface);
    border-radius: 1rem;
    padding: 2rem;
    width: 100%;
    max-width: 600px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.clone-header {
    text-align: center;
    margin-bottom: 2rem;
}

.logo {
    display: flex;
    justify-content: center;
    margin-bottom: 1rem;
}

.clone-title {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
    margin-bottom: 0.5rem;
}

.clone-subtitle {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    margin: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
        "Liberation Mono", "Courier New", monospace;
}

.clone-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.form-group label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
}

.input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    flex: 1;
}

.input-wrapper svg {
    position: absolute;
    left: 1rem;
    color: var(--color-text-secondary);
}

.input-wrapper input {
    width: 100%;
    padding: 0.75rem 1rem 0.75rem 3rem;
    background-color: rgba(148, 163, 184, 0.1);
    border: 1px solid var(--color-border);
    border-radius: 0.5rem;
    color: var(--color-text);
    font-size: 0.875rem;
    transition: all 0.2s;
}

.input-wrapper input::placeholder {
    color: var(--color-text-secondary);
}

.input-wrapper input:focus {
    outline: none;
    border-color: var(--color-primary);
    background-color: rgba(148, 163, 184, 0.15);
}

.input-group {
    display: flex;
    gap: 0.5rem;
}

.browse-button {
    padding: 0.75rem;
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: 0.5rem;
    color: var(--color-text);
    cursor: pointer;
    transition: all 0.2s;
}

.browse-button:hover {
    background-color: rgba(148, 163, 184, 0.1);
}

.advanced-options {
    border-top: 1px solid var(--color-border);
    padding-top: 1rem;
}

.advanced-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: none;
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    cursor: pointer;
    padding: 0;
}

.advanced-toggle svg {
    transition: transform 0.2s;
}

.advanced-toggle svg.rotate {
    transform: rotate(180deg);
}

.clone-button {
    background-color: var(--color-primary);
    color: white;
    padding: 0.75rem;
    border: none;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
}

.clone-button:hover {
    background-color: #0ea472;
}

.clone-button:active {
    background-color: #0c8a5f;
}
</style> 