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
            policy = "fixed",
            width = 200,
            height = 100,
        },
        -- size = {
        --     policy = "initial_content",
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
                    color = 0xffff0000,
                    font = {
                        family = "sans-serif",
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
                    font = {
                        family = "sans-serif",
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
                    font = {
                        family = "sans-serif",
                        size = 12.0,
                    },
                },
            },
        },
    },
}
