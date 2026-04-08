// Auto-dismiss toast notifications
function dismissToasts() {
    document.querySelectorAll('[data-auto-dismiss]').forEach((toast) => {
        const delay = parseInt(toast.dataset.autoDismiss);
        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transition = 'opacity 0.3s';
            setTimeout(() => toast.remove(), 300);
        }, delay);
    });
}

document.addEventListener('htmx:afterSwap', dismissToasts);
window.addEventListener('DOMContentLoaded', dismissToasts);
