<script setup lang="ts">
import { ref, defineEmits } from "vue";

const emit = defineEmits<{
    (e: "login", credentials: { username: string; password: string }): void;
}>();

const username = ref("");
const password = ref("");

const handleSubmit = () => {
    emit("login", {
        username: username.value,
        password: password.value,
    });
};
</script>

<template>
    <div class="login-container">
        <div class="login-card">
            <div class="login-header">
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
                    <h1 class="app-title">Branchie</h1>
                </div>
                <p class="app-subtitle">Your favorite Git GUI companion</p>
            </div>

            <form class="login-form" @submit.prevent="handleSubmit">
                <div class="form-group">
                    <label for="username">Username</label>
                    <div class="input-wrapper">
                        <svg
                            width="20"
                            height="20"
                            viewBox="0 0 20 20"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M16.6666 17.5V15.8333C16.6666 14.9493 16.3155 14.1014 15.6903 13.4763C15.0652 12.8512 14.2174 12.5 13.3333 12.5H6.66665C5.78259 12.5 4.93474 12.8512 4.30962 13.4763C3.6845 14.1014 3.33331 14.9493 3.33331 15.8333V17.5"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                            <path
                                d="M10 9.16667C11.841 9.16667 13.3333 7.67428 13.3333 5.83333C13.3333 3.99238 11.841 2.5 10 2.5C8.15905 2.5 6.66666 3.99238 6.66666 5.83333C6.66666 7.67428 8.15905 9.16667 10 9.16667Z"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        <input
                            id="username"
                            v-model="username"
                            type="text"
                            placeholder="Enter username"
                            required
                        />
                    </div>
                </div>

                <div class="form-group">
                    <label for="password">Password</label>
                    <div class="input-wrapper">
                        <svg
                            width="20"
                            height="20"
                            viewBox="0 0 20 20"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M15.8333 9.16667H4.16667C3.24619 9.16667 2.5 9.91286 2.5 10.8333V16.6667C2.5 17.5871 3.24619 18.3333 4.16667 18.3333H15.8333C16.7538 18.3333 17.5 17.5871 17.5 16.6667V10.8333C17.5 9.91286 16.7538 9.16667 15.8333 9.16667Z"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                            <path
                                d="M5.83331 9.16667V5.83333C5.83331 4.72827 6.27231 3.66846 7.05371 2.88706C7.83511 2.10566 8.89492 1.66667 9.99998 1.66667C11.105 1.66667 12.1649 2.10566 12.9463 2.88706C13.7277 3.66846 14.1666 4.72827 14.1666 5.83333V9.16667"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        <input
                            id="password"
                            v-model="password"
                            type="password"
                            placeholder="Enter password"
                            required
                        />
                    </div>
                </div>

                <button type="submit" class="login-button">Login</button>
            </form>
        </div>
    </div>
</template>

<style scoped>
.login-container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-background);
    padding: 1rem;
}

.login-card {
    background-color: var(--color-surface);
    border-radius: 1rem;
    padding: 2rem;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1),
        0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.login-header {
    text-align: center;
    margin-bottom: 2rem;
}

.logo {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
}

.app-title {
    font-size: 1.875rem;
    font-weight: 600;
    color: var(--color-text);
    margin: 0;
}

.app-subtitle {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    margin: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
        "Liberation Mono", "Courier New", monospace;
}

.login-form {
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

.login-button {
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

.login-button:hover {
    background-color: #0ea472;
}

.login-button:active {
    background-color: #0c8a5f;
}
</style> 