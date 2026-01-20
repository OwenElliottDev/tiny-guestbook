const form = document.getElementById('review-form');
const formStatus = document.getElementById('form-status');

form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const data = {
        name: form.name.value.trim(),
        email: form.email.value.trim() || null,
        rating: parseInt(form.rating.value),
        note: form.note.value.trim(),
    };

    if (!data.name || !data.rating || !data.note) {
        formStatus.textContent = 'Please fill in all required fields.';
        formStatus.style.color = 'red';
        return;
    }

    try {
        const response = await fetch('/api/sign', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data),
        });

        if (response.ok) {
            formStatus.textContent = 'Your review has been submitted! ✅';
            formStatus.style.color = 'green';
            form.reset();
            window.location.href = '/';
        } else {
            const err = await response.text();
            formStatus.textContent = 'Error: ' + err;
            formStatus.style.color = 'red';
        }
    } catch (err) {
        formStatus.textContent = 'Network error. Please try again.';
        formStatus.style.color = 'red';
    }
});
