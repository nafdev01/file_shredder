const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;


function formatTimestamp(searched_at) {
    // Create a Date object from the timestamp string
    let date = new Date(searched_at);

    // Extract the hours, minutes, day, month, and year
    let hours = date.getHours();
    let minutes = date.getMinutes();
    let day = date.getDate();
    let month = date.getMonth() + 1; // getMonth() is zero-based
    let year = date.getFullYear();

    // Determine whether it's AM or PM
    let period = hours >= 12 ? 'PM' : 'AM';

    // Convert the hours to a 12-hour format
    hours = hours % 12;
    hours = hours ? hours : 12; // the hour '0' should be '12'

    // Pad the hours, minutes, day, and month with leading zeros if necessary
    hours = hours < 10 ? '0' + hours : hours;
    minutes = minutes < 10 ? '0' + minutes : minutes;
    day = day < 10 ? '0' + day : day;
    month = month < 10 ? '0' + month : month;

    // Format the timestamp
    let formattedTimestamp = `${hours}:${minutes} ${period} on ${year}-${month}-${day}`;

    return formattedTimestamp;
}


if (document.querySelector('#dir-button')) {
    try {
        document.querySelector('#dir-button').addEventListener('click', function () {
            open({
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
            const employeeId = localStorage.getItem('employeeId');

            // Show the loading spinner
            Swal.fire({
                html: `<h3>Fetching files... <b></b></h3>`,
                didOpen: () => {
                    Swal.showLoading();
                    Swal.getPopup().querySelector("b");
                },
            })

            invoke('find_files', { pattern: pattern, directory: directory, searcher: employeeId }).then(files => {
                const resultsContainer = document.getElementById('results-container');
                resultsContainer.innerHTML = '';
                const directoryPath = document.getElementById('dir-path').value;

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
                        // Create an 'li' element for each file
                        const fileElement = document.createElement('li');
                        fileElement.className = 'list-group-item';

                        // Create a 'div' with Bootstrap classes for layout
                        const fileDiv = document.createElement('div');
                        fileDiv.className = 'd-flex justify-content-between';

                        // Create a 'p' element to store the file content
                        const fileText = document.createElement('p');
                        fileText.textContent = file;
                        fileText.className = 'mb-0'; // Add Bootstrap class for margin-bottom

                        // Create a 'span' element
                        const spanElement = document.createElement('span');
                        spanElement.className = 'ml-3'; // Add Bootstrap class for margin-left

                        // Create a 'button' and set its 'data-file' attribute to the file's full path
                        const buttonElement = document.createElement('button');
                        buttonElement.textContent = 'Request Shred';
                        buttonElement.setAttribute('data-file', `${file}`);
                        // add the shred-button class to the button
                        buttonElement.className = 'btn btn-danger';
                        // add the shredRequest function to the button onclick event

                        buttonElement.onclick = function (event) {
                            shredRequest(event.target); // event.target refers to the clicked button
                        };


                        // Append the button to the span
                        spanElement.appendChild(buttonElement);

                        // Append the fileText and spanElement to the fileDiv
                        fileDiv.appendChild(fileText);
                        fileDiv.appendChild(spanElement);

                        // Append the fileDiv to the fileElement
                        fileElement.appendChild(fileDiv);

                        // Append the fileElement to the listGroup
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



}


if (document.querySelector('#history-table')) {
    try {
        const historyTableBody = document.querySelector('#history-table-body');
        const userName = localStorage.getItem('employeeUsername');
        const employeeId = localStorage.getItem('employeeId');

        invoke('get_search_history', { searcher: employeeId }).then(history => {
            if (history.length === 0) {
                const noHistoryElement = document.createElement('p');
                noHistoryElement.textContent = 'No search history';
                historyTableBody.appendChild(noHistoryElement);
            }
            else {
                history.forEach(search => {
                    const row = document.createElement('tr');

                    const wordCell = document.createElement('th');
                    wordCell.textContent = search.word;
                    wordCell.classList.add('cell');
                    wordCell.setAttribute('scope', 'row');
                    row.appendChild(wordCell);

                    const directoryCell = document.createElement('td');
                    directoryCell.textContent = search.directory;
                    directoryCell.classList.add('cell');
                    row.appendChild(directoryCell);

                    const noFilesCell = document.createElement('td');
                    noFilesCell.textContent = search.no_of_files;
                    noFilesCell.classList.add('cell');
                    row.appendChild(noFilesCell);

                    const searchedAtCell = document.createElement('td');
                    searchedAtCell.textContent = formatTimestamp(search.searched_at);
                    searchedAtCell.classList.add('cell');
                    row.appendChild(searchedAtCell);


                    historyTableBody.appendChild(row);
                });
            }
        }).catch(error => {
            Swal.fire({
                icon: 'error',
                title: error,
            });
        });

    } catch (error) {
        // output the error
        Swal.fire({
            icon: 'error',
            title: error.message
        });
    }
}

function shredRequest(shredButton) {
    filepath = shredButton.getAttribute('data-file');

    const employeeId = localStorage.getItem('employeeId');

    Swal.fire({
        title: 'Are you sure?',
        html: `You are about to submit a shred request for the file: <b>${filepath}</b>`,
        icon: 'warning',
        showCancelButton: true,
        confirmButtonText: `Submit&nbsp;<i class="fas fa-arrow-right"></i>`,
        cancelButtonText: 'No, cancel!',
        reverseButtons: true,
        customClass: {
            confirmButton: "btn app-btn-success mx-2",
            cancelButton: "btn app-btn-danger mx-2"
        },
        buttonsStyling: false,
    }).then((result) => {
        if (result.isConfirmed) {
            invoke('create_shred_request', { requestby: employeeId, filepath: filepath }).then(response => {
                Swal.fire({
                    icon: 'success',
                    title: 'Request submitted!',
                    html: `Your request to shred file <span class="text-warning fw-bold">${filepath}</span> has been submitted sucessfully!`,
                    text: response,
                });
            }).catch(error => {
                Swal.fire({
                    icon: 'error',
                    title: error,
                });
            });
        }
    })
}