console.log('[Toast.js] Script loaded');

// Auto-dismiss toast notifications
document.addEventListener('htmx:afterSwap', () => {
    console.log('[Toast.js] htmx:afterSwap event fired');
    const toasts = document.querySelectorAll('[data-auto-dismiss]');
    console.log('[Toast.js] Found', toasts.length, 'toasts with data-auto-dismiss');

    toasts.forEach((toast, index) => {
        const delay = parseInt(toast.dataset.autoDismiss);
        console.log(`[Toast.js] Toast ${index}: id=${toast.id}, delay=${delay}ms`);

        setTimeout(() => {
            console.log(`[Toast.js] Dismissing toast ${index} (${toast.id})`);
            toast.style.opacity = '0';
            toast.style.transition = 'opacity 0.3s';
            setTimeout(() => {
                console.log(`[Toast.js] Removing toast ${index} (${toast.id})`);
                toast.remove();
            }, 300);
        }, delay);
    });
});

// Also run on page load for any existing toasts
window.addEventListener('DOMContentLoaded', () => {
    console.log('[Toast.js] DOMContentLoaded event fired');
    const toasts = document.querySelectorAll('[data-auto-dismiss]');
    console.log('[Toast.js] Found', toasts.length, 'toasts on page load');

    toasts.forEach((toast, index) => {
        const delay = parseInt(toast.dataset.autoDismiss);
        console.log(`[Toast.js] Page load toast ${index}: id=${toast.id}, delay=${delay}ms`);

        setTimeout(() => {
            console.log(`[Toast.js] Dismissing page load toast ${index} (${toast.id})`);
            toast.style.opacity = '0';
            toast.style.transition = 'opacity 0.3s';
            setTimeout(() => {
                console.log(`[Toast.js] Removing page load toast ${index} (${toast.id})`);
                toast.remove();
            }, 300);
        }, delay);
    });
});
