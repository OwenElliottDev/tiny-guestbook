console.log("JS loaded successfully!");

document.addEventListener("DOMContentLoaded", () => {
    console.log("DOM ready");
});


async function getGuestBookEntries(sortOrder) {
    try {
        const url = `/api/entries?sort=${sortOrder}`;
        const res = await fetch(url);
        if (!res.ok) throw new Error(`HTTP error! status: ${res.status}`);

        const data = await res.json();

        const container = document.getElementById('entries-container');
        container.innerHTML = "";

        data.entries.forEach(entry => {
            const card = document.createElement('div');
            card.className = 'entry-card';

            card.innerHTML = `
                <div class="entry-left">
                    <p class="entry-name"><strong>${entry.name}</strong></p>
                    <p class="entry-rating">Rating: ${entry.rating}</p>
                    <p class="entry-date">${new Date(entry.posted_at_utc * 1000).toLocaleString()}</p>
                </div>
                <div class="entry-right">
                    <p class="entry-note">${entry.note}</p>
                </div>
            `;

            container.appendChild(card);

            const note = card.querySelector('.entry-note');

            const lineHeight = parseFloat(getComputedStyle(note).lineHeight);
            const maxHeight = lineHeight * 5;

            if (note.scrollHeight > maxHeight + 1) {
                note.style.maxHeight = `${maxHeight}px`;
                note.style.overflow = 'hidden';

                const btn = document.createElement('button');
                btn.className = 'show-more-btn';
                btn.textContent = 'Show more...';

                btn.addEventListener('click', () => {
                    if (note.style.maxHeight) {
                        note.style.maxHeight = '';
                        btn.textContent = 'Show less...';
                    } else {
                        note.style.maxHeight = `${maxHeight}px`;
                        btn.textContent = 'Show more...';
                    }
                });

                card.querySelector('.entry-right').appendChild(btn);
            }
        });

    } catch (err) {
        console.error("Failed to fetch entries:", err);
        alert("Failed to fetch entries. See console for details.");
    }
}

getGuestBookEntries("datePostedDesc");