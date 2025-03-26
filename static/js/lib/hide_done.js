document.getElementById('hide-done-btn').addEventListener('click', () => {
    document.querySelectorAll('.completed').forEach(item => {
        item.classList.toggle('hidden');
    });
});
