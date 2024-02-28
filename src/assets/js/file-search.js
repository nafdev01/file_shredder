const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification
const dialog = window.__TAURI__.dialog

try {
    document.querySelector('#dir-button').addEventListener('click', function () {
        dialog.open({
            directory: true,
            defaultPath: document.getElementById('dir-path').value
        }).then(directory => {
            document.getElementById('dir-path').value = `${directory}`;
        }).catch(error => {
            console.error(error);
        });
    });
} catch (error) {
    Swal.fire({
        icon: 'error',
        title: 'Error',
        text: error.message
    });
}


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
        const directory = document.getElementById('dir-path').value;
        const userName = localStorage.getItem('employeeUsername');

        
        // Show the loading spinner
        Swal.fire({
            html: `<h3>Fetching files... <b></b></h3>`,
            timer: 3000,
            didOpen: () => {
                Swal.showLoading();
                Swal.getPopup().querySelector("b");
            },
        })

        window.__TAURI__.invoke('find_files', { pattern: pattern, directory: directory, searcher: userName }).then(files => {
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
        }).catch(error => {
            Swal.fire({
                icon: 'error',
                title: error,
            });
        });
    });
} catch (error) {
    // output the error
    Swal.fire({
        icon: 'error',
        title: error.message
    });
}
