document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('shorten-form');
    const urlInput = document.getElementById('url-input');
    const resultContainer = document.getElementById('result-container');
    const shortenedUrl = document.getElementById('shortened-url');
    const copyBtn = document.getElementById('copy-btn');
    const errorContainer = document.getElementById('error-container');
    const errorMessage = document.getElementById('error-message');

    // The backend is running locally on port 8000
    const API_URL = 'http://localhost:8000';

    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const url = urlInput.value.trim();

        if (!url) return;

        // Reset UI
        resultContainer.classList.add('hidden');
        errorContainer.classList.add('hidden');
        
        const submitBtn = form.querySelector('button[type="submit"]');
        const originalBtnText = submitBtn.textContent;
        submitBtn.textContent = 'Shortening...';
        submitBtn.disabled = true;

        try {
            const response = await fetch(`${API_URL}/`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ url })
            });

            if (!response.ok) {
                throw new Error(`Server responded with status: ${response.status}`);
            }

            const data = await response.json();
            
            const shortLink = `${API_URL}/${data.short_code}`;
            shortenedUrl.href = shortLink;
            shortenedUrl.textContent = shortLink;
            
            resultContainer.classList.remove('hidden');
        } catch (error) {
            console.error('Error shortening URL:', error);
            errorMessage.textContent = 'Failed to shorten URL. Is the backend running?';
            errorContainer.classList.remove('hidden');
        } finally {
            submitBtn.textContent = originalBtnText;
            submitBtn.disabled = false;
        }
    });

    copyBtn.addEventListener('click', () => {
        const url = shortenedUrl.textContent;
        navigator.clipboard.writeText(url).then(() => {
            const originalHTML = copyBtn.innerHTML;
            copyBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#50e3c2" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>';
            setTimeout(() => {
                copyBtn.innerHTML = originalHTML;
            }, 2000);
        }).catch(err => {
            console.error('Failed to copy text: ', err);
        });
    });
});
