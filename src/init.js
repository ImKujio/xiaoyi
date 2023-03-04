window.addEventListener('keydown', function(event) {
    if (event.ctrlKey && event.key === 'f') {
        event.preventDefault();
    }
});

// 阻止 Ctrl + G
window.addEventListener('keydown', function(event) {
    if (event.ctrlKey && event.key === 'g') {
        event.preventDefault();
    }
});

// 阻止 Ctrl + P
window.addEventListener('keydown', function(event) {
    if (event.ctrlKey && event.key === 'p') {
        event.preventDefault();
    }
});