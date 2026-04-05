// Auto-dismiss toast notifications
document.addEventListener('htmx:afterSwap', () => {
    document.querySelectorAll('[data-auto-dismiss]').forEach(toast => {
        const delay = parseInt(toast.dataset.autoDismiss);
        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transition = 'opacity 0.3s';
            setTimeout(() => toast.remove(), 300);
        }, delay);
    });
});

// Also run on page load for any existing toasts
window.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('[data-auto-dismiss]').forEach(toast => {
        const delay = parseInt(toast.dataset.autoDismiss);
        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transition = 'opacity 0.3s';
            setTimeout(() => toast.remove(), 300);
        }, delay);
    });
});
