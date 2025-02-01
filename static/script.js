async function shortenUrl() {
    const longUrl = document.getElementById("longUrl").value;
    const response = await fetch("/shorten", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ long_url: longUrl })
    });

    const data = await response.json();
    document.getElementById("shortUrl").innerText = `Short URL: ${data.short_url}`;
}
