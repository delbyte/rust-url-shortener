// script.js
async function shortenUrl() {
    const longUrl = document.getElementById("longUrl").value.trim();
    const resultElement = document.getElementById("shortUrl");
    const button = document.querySelector('button');
    
    // Basic validation
    if (!longUrl) {
        resultElement.innerHTML = '<span style="color: red;">Please enter a URL</span>';
        return;
    }

    try {
        // Disable button during request
        button.disabled = true;
        resultElement.innerHTML = 'Processing...';

        const response = await fetch("/shorten", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ long_url: longUrl })
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Failed to shorten URL');
        }

        // Create clickable link
        const shortUrl = data.short_url;
        resultElement.innerHTML = `Short URL: <a href="${shortUrl}" target="_blank">${shortUrl}</a>`;
    } catch (error) {
        resultElement.innerHTML = `<span style="color: red;">Error: ${error.message}</span>`;
    } finally {
        button.disabled = false;
    }
}

// Add enter key support
document.getElementById("longUrl").addEventListener("keypress", function(event) {
    if (event.key === "Enter") {
        event.preventDefault();
        shortenUrl();
    }
});