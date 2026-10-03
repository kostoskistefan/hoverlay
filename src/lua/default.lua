return {
    viewport = {
        anchor = {
            horizontal = "right",
            vertical = "bottom",
        },
        offset = {
            horizontal = 12,
            vertical = 8,
        },
        size = {
            policy = "initial_content",
        },
        -- size = {
        --     policy = "fixed",
        --     width = 200,
        --     height = 100,
        -- },
    },
    container = {
        margin = 6,
        spacing = 6,
        background = 0x00000000,
        children = {
            {
                content = "Hello!",
                style = {
                    alignment = "left",
                    color = 0xffffffff,
                    shadow_color = 0xff000000,
                    font = {
                        family = "Roboto",
                        size = 12.0,
                    },
                },
            },
            {
                content = function()
                    return os.date("%H:%M:%S")
                end,
                style = {
                    alignment = "center",
                    color = 0xff00ff00,
                    shadow_color = 0xff000000,
                    font = {
                        family = "Inter",
                        size = 12.0,
                    },
                },
            },
            {
                content = function()
                    return os.date("%a %d %b")
                end,
                style = {
                    alignment = "right",
                    color = 0xff0000ff,
                    shadow_color = 0xff000000,
                    font = {
                        family = "font-that-doesnt-exist-falls-back-to-noto-sans",
                        size = 12.0,
                    },
                },
            },
        },
    },
}
