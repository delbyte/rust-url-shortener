async function shortenUrl() {
    const longUrl = document.getElementById("longUrl").value.trim();
    const resultElement = document.getElementById("shortUrl");
    const qrImage = document.getElementById("qrCode");
    const resultContainer = document.getElementById("result");
    const button = document.querySelector('button');

    // Basic validation
    if (!longUrl) {
        resultElement.innerHTML = '<span style="color: red;">Please enter a URL</span>';
        qrImage.classList.add("hidden");
        return;
    }

    try {
        // Disable button during request
        button.disabled = true;
        resultElement.innerHTML = 'Processing...';
        qrImage.classList.add("hidden");

        const response = await fetch("/shorten", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ long_url: longUrl })
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || 'Failed to shorten URL');
        }

        // Create clickable short link
        const shortUrl = data.short_url;
        resultElement.innerHTML = `Short URL: <a href="${shortUrl}" target="_blank">${shortUrl}</a>`;
        resultContainer.classList.remove("hidden");

        // Generate and display QR code for the long URL
        generateQRCode(longUrl); 

    } catch (error) {
        resultElement.innerHTML = `<span style="color: red;">Error: ${error.message}</span>`;
        qrImage.classList.add("hidden");
    } finally {
        button.disabled = false;
    }
}


async function generateQRCode(url) {
    const qrImage = document.getElementById("qrCode");

    try {
        const qrResponse = await fetch(`/qr?url=${encodeURIComponent(url)}`);
        if (!qrResponse.ok) {
            throw new Error("Failed to generate QR code");
        }

        const data = await qrResponse.json(); 
        console.log('QR Code Data:', data);  
        qrImage.src = data;  
        qrImage.classList.remove("hidden");
    } catch (error) {
        console.error("QR Code Error:", error);
    }
}


// Add enter key support
document.getElementById("longUrl").addEventListener("keypress", function(event) {
    if (event.key === "Enter") {
        event.preventDefault();
        shortenUrl();
    }
});
