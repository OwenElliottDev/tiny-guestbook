console.log("JS loaded successfully!");

document.addEventListener("DOMContentLoaded", () => {
    console.log("DOM ready");
});


const PAGE_SIZE = 32;

let currentPage = 0;
let currentSort = "datePostedDesc";

async function getGuestBookEntries(sortOrder, page = 0) {
    currentSort = sortOrder;
    currentPage = page;
    try {
        const url = `/api/entries?sort=${sortOrder}&limit=${PAGE_SIZE}&offset=${page * PAGE_SIZE}`;
        const res = await fetch(url);
        if (!res.ok) throw new Error(`HTTP error! status: ${res.status}`);

        const data = await res.json();

        const container = document.getElementById('entries-container');
        container.innerHTML = "";

        renderPagination(data.total);

        if (data.entries.length === 0) {
            container.innerHTML = '<p class="empty-state">No entries yet — be the first to sign!</p>';
            return;
        }

        data.entries.forEach(entry => {
            const card = document.createElement('div');
            card.className = 'entry-card';

            card.innerHTML = `
                <div class="entry-left">
                    <p class="entry-name"><strong>${entry.name}</strong></p>
                    <p class="entry-rating">${'★'.repeat(entry.rating)}${'★'.repeat(5 - entry.rating).split('').map(() => '<span class="star-empty">★</span>').join('')}</p>
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

function renderPagination(total) {
    const pagination = document.getElementById('pagination');
    const totalPages = Math.ceil(total / PAGE_SIZE);

    if (totalPages <= 1) {
        pagination.hidden = true;
        return;
    }
    pagination.hidden = false;

    const prev = document.getElementById('prev-page');
    const next = document.getElementById('next-page');
    prev.disabled = currentPage === 0;
    next.disabled = currentPage >= totalPages - 1;
    document.getElementById('page-info').textContent = `Page ${currentPage + 1} of ${totalPages}`;
}

function goToPage(page) {
    getGuestBookEntries(currentSort, page);
    window.scrollTo({ top: 0, behavior: 'smooth' });
}

document.getElementById('prev-page').addEventListener('click', () => goToPage(currentPage - 1));
document.getElementById('next-page').addEventListener('click', () => goToPage(currentPage + 1));

getGuestBookEntries("datePostedDesc");