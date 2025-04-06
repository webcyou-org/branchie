<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Branch {
    id: string;
    name: string;
    isActive: boolean;
}

interface FileChange {
    id: string;
    name: string;
    status: "modified" | "added" | "deleted";
}

interface File {
    id: string;
    name: string;
    type: "file" | "directory";
    path: string;
}

interface Commit {
    id: string;
    hash: string;
    message: string;
    author: {
        name: string;
        avatar: string;
    };
    timestamp: string;
}

const isDarkMode = ref(
    window.matchMedia("(prefers-color-scheme: dark)").matches
);
const menuOpen = ref(false);
const currentTab = ref("history");
const currentBranch = ref("main");

const branches = ref<Branch[]>([
    { id: "main", name: "main", isActive: true },
    { id: "feature/auth", name: "feature/auth", isActive: false },
    { id: "hotfix/bug-123", name: "hotfix/bug-123", isActive: false },
]);

const changes = ref<FileChange[]>([
    { id: "change1", name: "src/components/Button.js", status: "modified" },
    { id: "change2", name: "src/styles/main.css", status: "deleted" },
]);

const files = ref<File[]>([
    { id: "file1", name: "src", type: "directory", path: "src" },
    { id: "file2", name: "index.js", type: "file", path: "src/index.js" },
    { id: "file3", name: "App.js", type: "file", path: "src/App.js" },
]);

const commits = ref<Commit[]>([
    {
        id: "1",
        hash: "8f4d2a3",
        message: "Update authentication flow",
        author: {
            name: "John Doe",
            avatar: "https://ui-avatars.com/api/?name=John+Doe&background=random",
        },
        timestamp: "2 hours ago",
    },
    {
        id: "2",
        hash: "3e7a1b9",
        message: "Fix styling issues in dashboard",
        author: {
            name: "Sarah Smith",
            avatar: "https://ui-avatars.com/api/?name=Sarah+Smith&background=random",
        },
        timestamp: "5 hours ago",
    },
]);

// ダークモードの切り替えを監視
window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (e) => {
        isDarkMode.value = e.matches;
    });

const toggleMenu = () => {
    menuOpen.value = !menuOpen.value;
};

const selectBranch = (branchId: string) => {
    currentBranch.value = branchId;
    branches.value = branches.value.map((branch: Branch) => ({
        ...branch,
        isActive: branch.id === branchId,
    }));
};
</script>

<template>
    <div class="app-container" :class="{ 'dark-mode': isDarkMode }">
        <header class="header">
            <div class="header-main">
                <div class="header-left">
                    <div class="logo">
                        <svg
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.58 20 4 16.42 4 12C4 7.58 7.58 4 12 4C16.42 4 20 7.58 20 12C20 16.42 16.42 20 12 20Z"
                                fill="#10B981"
                            />
                        </svg>
                        <span class="app-name">Branchie</span>
                    </div>
                    <div class="project-name">
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M2 4H14V12H2V4Z"
                                stroke="#64748B"
                                stroke-width="1.5"
                            />
                        </svg>
                        <span>my-awesome-project</span>
                    </div>
                </div>
                <div class="header-right">
                    <button class="btn btn-secondary">
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M8 3V13M8 3L5 6M8 3L11 6"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        Fetch
                    </button>
                    <button class="btn btn-primary">
                        <svg
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            fill="none"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M8 13V3M8 13L5 10M8 13L11 10"
                                stroke="currentColor"
                                stroke-width="1.5"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </svg>
                        Push
                    </button>
                </div>
            </div>
            <nav class="header-nav">
                <button
                    class="nav-button"
                    :class="{ active: currentTab === 'history' }"
                    @click="currentTab = 'history'"
                >
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 16 16"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M8 4V8L10 10"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        />
                        <circle
                            cx="8"
                            cy="8"
                            r="6"
                            stroke="currentColor"
                            stroke-width="1.5"
                        />
                    </svg>
                    History
                </button>
                <button
                    class="nav-button"
                    :class="{ active: currentTab === 'diff' }"
                    @click="currentTab = 'diff'"
                >
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 16 16"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            d="M4 8H12M4 4H12M4 12H8"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                        />
                    </svg>
                    Diff
                </button>
                <button
                    class="nav-button"
                    :class="{ active: currentTab === 'graph' }"
                    @click="currentTab = 'graph'"
                >
                    <svg
                        width="16"
                        height="16"
                        viewBox="0 0 16 16"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <circle
                            cx="4"
                            cy="4"
                            r="2"
                            stroke="currentColor"
                            stroke-width="1.5"
                        />
                        <circle
                            cx="4"
                            cy="12"
                            r="2"
                            stroke="currentColor"
                            stroke-width="1.5"
                        />
                        <circle
                            cx="12"
                            cy="8"
                            r="2"
                            stroke="currentColor"
                            stroke-width="1.5"
                        />
                        <path
                            d="M4 6V10M5.5 4.5L10.5 7.5"
                            stroke="currentColor"
                            stroke-width="1.5"
                        />
                    </svg>
                    Graph
                </button>
            </nav>
        </header>

        <div class="app-layout">
            <nav class="side-nav">
                <div class="nav-section">
                    <h2 class="nav-section-title">Branches</h2>
                    <ul class="branch-list">
                        <li
                            v-for="branch in branches"
                            :key="branch.id"
                            class="branch-item"
                            :class="{ active: branch.isActive }"
                            @click="selectBranch(branch.id)"
                        >
                            <svg
                                class="branch-icon"
                                width="16"
                                height="16"
                                viewBox="0 0 16 16"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <circle
                                    cx="8"
                                    cy="8"
                                    r="4"
                                    :fill="
                                        branch.isActive ? '#10B981' : '#64748B'
                                    "
                                />
                            </svg>
                            <span class="branch-name">{{ branch.name }}</span>
                        </li>
                    </ul>
                </div>

                <div class="nav-section">
                    <h2 class="nav-section-title">Changes</h2>
                    <ul class="file-list">
                        <li
                            v-for="change in changes"
                            :key="change.id"
                            class="file-item"
                        >
                            <svg
                                class="file-icon"
                                width="16"
                                height="16"
                                viewBox="0 0 16 16"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <circle
                                    cx="8"
                                    cy="8"
                                    r="4"
                                    :fill="
                                        change.status === 'modified'
                                            ? '#F59E0B'
                                            : change.status === 'added'
                                            ? '#10B981'
                                            : '#EF4444'
                                    "
                                />
                            </svg>
                            <span class="file-name">{{ change.name }}</span>
                        </li>
                    </ul>
                </div>

                <div class="nav-section">
                    <h2 class="nav-section-title">Files</h2>
                    <ul class="file-list">
                        <li
                            v-for="file in files"
                            :key="file.id"
                            class="file-item"
                        >
                            <svg
                                class="file-icon"
                                width="16"
                                height="16"
                                viewBox="0 0 16 16"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    :d="
                                        file.type === 'directory'
                                            ? 'M2 4h12v8H2V4z'
                                            : 'M3 2h7l3 3v9H3V2z'
                                    "
                                    stroke="currentColor"
                                    stroke-width="1.5"
                                    fill="none"
                                />
                            </svg>
                            <span class="file-name">{{ file.name }}</span>
                        </li>
                    </ul>
                </div>
            </nav>
            <main class="main-content">
                <div class="commit-list">
                    <div
                        v-for="commit in commits"
                        :key="commit.id"
                        class="commit-item"
                    >
                        <div class="commit-avatar">
                            <img
                                :src="commit.author.avatar"
                                :alt="commit.author.name"
                            />
                        </div>
                        <div class="commit-info">
                            <div class="commit-header">
                                <h3 class="commit-message">
                                    {{ commit.message }}
                                </h3>
                                <span class="commit-hash">{{
                                    commit.hash
                                }}</span>
                            </div>
                            <div class="commit-meta">
                                <span class="commit-author">{{
                                    commit.author.name
                                }}</span>
                                <span class="commit-separator">•</span>
                                <span class="commit-time">{{
                                    commit.timestamp
                                }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        </div>

        <footer class="footer">
            <div class="footer-content">
                <div class="footer-branch">
                    <svg
                        width="12"
                        height="12"
                        viewBox="0 0 16 16"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                        class="footer-icon"
                    >
                        <circle cx="8" cy="8" r="4" fill="currentColor" />
                    </svg>
                    <span class="footer-text">{{ currentBranch }}</span>
                </div>
            </div>
        </footer>
    </div>
</template>

<style>
:root {
    --color-background: #1a1f2e;
    --color-surface: #242b3d;
    --color-primary: #10b981;
    --color-secondary: #64748b;
    --color-text: #f8fafc;
    --color-text-secondary: #94a3b8;
    --color-border: rgba(148, 163, 184, 0.1);
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
        Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    background-color: var(--color-background);
    color: var(--color-text);
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
}

.app-container {
    min-height: 100vh;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.header {
    background-color: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
}

.header-main {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--color-border);
}

.header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.logo {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.app-name {
    font-weight: 600;
    font-size: 1.125rem;
}

.project-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    padding-left: 1rem;
    border-left: 1px solid var(--color-border);
}

.header-right {
    display: flex;
    gap: 0.5rem;
}

.btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s;
}

.btn-primary {
    background-color: var(--color-primary);
    color: white;
}

.btn-primary:hover {
    background-color: #0ea472;
}

.btn-secondary {
    background-color: var(--color-surface);
    color: var(--color-text);
    border: 1px solid var(--color-border);
}

.btn-secondary:hover {
    background-color: rgba(148, 163, 184, 0.1);
}

.header-nav {
    display: flex;
    gap: 1rem;
    padding: 0 1rem;
}

.nav-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
}

.nav-button:hover {
    color: var(--color-text);
}

.nav-button.active {
    color: var(--color-text);
    border-bottom-color: var(--color-primary);
}

.app-layout {
    display: flex;
    flex: 1;
    min-height: 0; /* flexboxのスクロール問題を解決 */
}

.side-nav {
    width: 240px;
    background-color: var(--color-surface);
    border-right: 1px solid var(--color-border);
    padding: 1rem;
    flex-shrink: 0;
    overflow-y: auto;
    /* カスタムスクロールバー */
    scrollbar-width: thin;
    scrollbar-color: var(--color-secondary) transparent;
}

.nav-section {
    margin-bottom: 1.5rem;
}

.nav-section-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--color-text-secondary);
    margin-bottom: 0.75rem;
    padding: 0 0.5rem;
}

.branch-list {
    list-style: none;
}

.branch-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    cursor: pointer;
    color: var(--color-text-secondary);
    transition: all 0.2s;
    font-size: 0.875rem;
}

.branch-item:hover {
    background-color: rgba(148, 163, 184, 0.1);
    color: var(--color-text);
}

.branch-item.active {
    background-color: rgba(16, 185, 129, 0.1);
    color: var(--color-text);
}

.branch-icon {
    flex-shrink: 0;
}

.branch-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.main-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    background-color: var(--color-background);
    /* カスタムスクロールバー */
    scrollbar-width: thin;
    scrollbar-color: var(--color-secondary) transparent;
}

.commit-list {
    background-color: var(--color-surface);
    border-radius: 0.5rem;
    border: 1px solid var(--color-border);
}

.commit-item {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    border-bottom: 1px solid var(--color-border);
    transition: background-color 0.2s;
}

.commit-item:last-child {
    border-bottom: none;
}

.commit-item:hover {
    background-color: rgba(148, 163, 184, 0.1);
}

.commit-avatar {
    flex-shrink: 0;
}

.commit-avatar img {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 50%;
    object-fit: cover;
}

.commit-info {
    flex: 1;
    min-width: 0; /* 子要素の text-overflow を有効にするため */
}

.commit-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    margin-bottom: 0.25rem;
}

.commit-message {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.commit-hash {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
        "Liberation Mono", "Courier New", monospace;
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    flex-shrink: 0;
}

.commit-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
    color: var(--color-text-secondary);
}

.commit-author {
    font-weight: 500;
}

.commit-separator {
    color: var(--color-text-secondary);
    opacity: 0.5;
}

.commit-time {
    color: var(--color-text-secondary);
}

.footer {
    background-color: var(--color-surface);
    border-top: 1px solid var(--color-border);
    height: 24px;
    display: flex;
    align-items: center;
}

.footer-content {
    padding: 0 0.75rem;
    display: flex;
    align-items: center;
    gap: 1rem;
}

.footer-branch {
    display: flex;
    align-items: center;
    gap: 0.375rem;
}

.footer-icon {
    color: var(--color-primary);
}

.footer-text {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
}

.dark-mode .footer {
    background-color: var(--color-surface);
}

@media (max-width: 768px) {
    .menu-button {
        display: block;
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.5rem;
    }

    .menu-icon {
        display: block;
        width: 24px;
        height: 2px;
        background-color: var(--text-light);
        position: relative;
        transition: background-color var(--transition-speed);
    }

    .dark-mode .menu-icon {
        background-color: var(--text-dark);
    }

    .menu-icon::before,
    .menu-icon::after {
        content: "";
        position: absolute;
        width: 24px;
        height: 2px;
        background-color: var(--text-light);
        transition: transform var(--transition-speed);
    }

    .dark-mode .menu-icon::before,
    .dark-mode .menu-icon::after {
        background-color: var(--text-dark);
    }

    .menu-icon::before {
        transform: translateY(-6px);
    }

    .menu-icon::after {
        transform: translateY(6px);
    }

    .nav-links {
        display: none;
        position: absolute;
        top: 100%;
        left: 0;
        width: 100%;
        background-color: var(--background-light);
        padding: 1rem;
        flex-direction: column;
        gap: 1rem;
        box-shadow: 0 2px 4px var(--shadow-color);
    }

    .dark-mode .nav-links {
        background-color: var(--background-dark);
    }

    .nav-menu.open .nav-links {
        display: flex;
    }

    .hero-section h2 {
        font-size: 2rem;
    }

    .cta-buttons {
        flex-direction: column;
    }

    .features-section {
        grid-template-columns: 1fr;
    }
}

.file-list {
    list-style: none;
}

.file-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 0.375rem;
    cursor: pointer;
    color: var(--color-text-secondary);
    transition: all 0.2s;
    font-size: 0.875rem;
}

.file-item:hover {
    background-color: rgba(148, 163, 184, 0.1);
    color: var(--color-text);
}

.file-icon {
    flex-shrink: 0;
}

.file-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

/* アイコンの色を設定 */
.file-item[data-status="modified"] .file-icon {
    color: #f59e0b;
}

.file-item[data-status="added"] .file-icon {
    color: #10b981;
}

.file-item[data-status="deleted"] .file-icon {
    color: #ef4444;
}

/* Webkit（Chrome、Safari等）用のカスタムスクロールバー */
.side-nav::-webkit-scrollbar,
.main-content::-webkit-scrollbar {
    width: 6px;
}

.side-nav::-webkit-scrollbar-track,
.main-content::-webkit-scrollbar-track {
    background: transparent;
}

.side-nav::-webkit-scrollbar-thumb,
.main-content::-webkit-scrollbar-thumb {
    background-color: var(--color-secondary);
    border-radius: 3px;
}

/* スクロールバーを必要な時だけ表示 */
.side-nav::-webkit-scrollbar-thumb,
.main-content::-webkit-scrollbar-thumb {
    background-color: transparent;
}

.side-nav:hover::-webkit-scrollbar-thumb,
.main-content:hover::-webkit-scrollbar-thumb {
    background-color: var(--color-secondary);
}
</style>