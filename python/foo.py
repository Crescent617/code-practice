import json

config = r""" {
    "window.menuBarVisibility": "toggle",
    "editor.quickSuggestions": {
        "other": true,
        "comments": true,
        "strings": true
    },
    "python.linting.pycodestyleCategorySeverity.E": "Warning",
    "python.linting.pycodestyleCategorySeverity.W": "Information",
    "editor.acceptSuggestionOnEnter": "off",
    "window.restoreWindows": "none",
    "editor.fontLigatures": true,
    "git.autorefresh": false,
    "editor.suggestSelection": "first",
    "vsintellicode.modify.editor.suggestSelection": "automaticallyOverrodeDefaultValue",
    "terminal.integrated.shell.windows": "C:\\WINDOWS\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
    "files.exclude": {
        "**/.ipynb_checkpoints": true,
        "**/.mypy_cache": true,
        "**/.classpath": true,
        "**/.project": true,
        "**/.settings": true,
        "**/.factorypath": true
    },
    "python.linting.flake8Args": [
        "--ignore=W291,W292,E226,E305,F401,W391,E501,W503,E203"
    ],
    "files.trimTrailingWhitespace": true,
    "python.formatting.provider": "black",
    "python.formatting.blackArgs": ["-S"],
    "[markdown]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[jsonc]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[html]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[javascript]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "prettier.semi": false,
    "C_Cpp.updateChannel": "Insiders",
    "python.defaultInterpreterPath": "C:\\Users\\lee\\scoop\\apps\\miniconda3\\current\\envs\\Hogwarts\\python.exe",
    "terminal.integrated.shell.linux": "/bin/bash",
    "emmet.includeLanguages": {
        "vue-html": "html",
        "javascript": "javascriptreact"
    },
    "[json]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "emmet.showExpandedAbbreviation": "inMarkupAndStylesheetFilesOnly",
    "emmet.triggerExpansionOnTab": true,
    "jshint.nodePath": "/home/lee_wsl/download/node-v12.18.3-linux-x64/bin/jshint",
    "jshint.enable": false,
    "prettier.tabWidth": 4,
    "prettier.printWidth": 90,
    "webhint.enableTelemetry": "disabled",
    "python.languageServer": "Pylance",
    "editor.formatOnSaveMode": "modifications",
    "debug.onTaskErrors": "abort",
    "code-runner.runInTerminal": true,
    "code-runner.saveFileBeforeRun": true,
    "vscode-neovim.neovimExecutablePaths.win32": "nvim",
    "vscode-neovim.neovimInitVimPaths.win32": "C:\\Users\\lee\\AppData\\Local\\nvim\\init.vim",
    "vscode-neovim.useCtrlKeysForInsertMode": false,
    "cSpell.userWords": [
        "Cacher",
        "Hasher",
        "Inorder",
        "Treap",
        "Uninit",
        "Wifi",
        "bitree",
        "btree",
        "canonicalize",
        "changgeng",
        "concat",
        "cooldown",
        "deque",
        "groupby",
        "indegree",
        "inplace",
        "ipdb",
        "itertools",
        "lowbit",
        "nexts",
        "nums",
        "postorder",
        "powf",
        "prepend",
        "ptrs",
        "pyplot",
        "rbtree",
        "segtree",
        "seqs",
        "ssid",
        "strs",
        "timeit",
        "vals",
        "vec",
        "vec deque",
        "vectorize"
    ],
    "vscode-neovim.textDecorationsAtTop": true,
    "editor.suggest.showStatusBar": true,
    "editor.tokenColorCustomizations": {
        "keywords": { "fontStyle": "italic" }
    },
    "workbench.colorCustomizations": {
        "[Dracula Soft]": {
            "sideBar.background": "#1e1e29",
            "activityBar.background": "#282A36",
            "titleBar.activeBackground": "#282A36",
            "statusBarItem.remoteForeground": "#000000"
        }
    },
    "rust-analyzer.completion.addCallArgumentSnippets": false,
    "rust-analyzer.inlayHints.parameterHints": false,
    "workbench.iconTheme": "material-icon-theme",
    "C_Cpp.clang_format_fallbackStyle": "{ BasedOnStyle: LLVM, UseTab: Never, IndentWidth: 4, TabWidth: 4, ColumnLimit: 0 }",
    "workbench.list.smoothScrolling": true,
    "workbench.colorTheme": "Dracula Soft",
    "editor.smoothScrolling": true,
    "window.zoomLevel": 0,
    "search.followSymlinks": false,
    "editor.minimap.showSlider": "always",
    "editor.minimap.renderCharacters": false,
    "editor.minimap.enabled": true,
    "python.linting.flake8Enabled": true,
    "python.linting.pylintEnabled": false
}
 """

js = json.loads(config)

with open('test.json', 'w') as f:
    json.dump(js, f, ensure_ascii=False, sort_keys=True)