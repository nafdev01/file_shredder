try {
    document.getElementById('file-search').addEventListener('keypress', function (event) {
        if (event.keyCode == 13) {
            event.preventDefault();
            document.getElementById('search-button').click();
        }
    });
} catch (error) {
    Swal.fire({
        icon: 'error',
        title: 'Error',
        text: error.message
    });
}

try {
    document.getElementById('search-button').addEventListener('click', () => {
        const pattern = document.getElementById('file-search').value;

        // Show the loading spinner
        Swal.fire({
            title: 'Searching...',
            allowOutsideClick: false,
            onBeforeOpen: () => {
                Swal.showLoading();
            },
        });

        window.__TAURI__.invoke('find_files', { pattern }).then(files => {
            const resultsContainer = document.getElementById('results-container');
            resultsContainer.innerHTML = '';

            // check if any files were found
            if (files.length === 0) {
                const noResultsElement = document.createElement('p');
                noResultsElement.textContent = 'No files found';
                resultsContainer.appendChild(noResultsElement);
            }
            else {
                const listGroup = document.createElement('ul');
                listGroup.className = 'list-group';

                files.forEach(file => {
                    const fileElement = document.createElement('li');
                    fileElement.className = 'list-group-item';
                    fileElement.textContent = file;
                    listGroup.appendChild(fileElement);
                });

                resultsContainer.appendChild(listGroup);
            }

            // Close the loading spinner
            Swal.close();
        });
    });
} catch (error) {
    Swal.fire({
        icon: 'error',
        title: 'Error',
        text: error.message
    });
}
