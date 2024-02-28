const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification
const dialog = window.__TAURI__.dialog

if (localStorage.getItem('adminId')) {
    const adminId = localStorage.getItem('adminId');

    try {
        invoke('get_shred_requests', { requestto: adminId }).then(shredRequests => {
            const shredRequestTable = document.getElementById('shred-request-table');
            let tableContent = `
                <thead>
                    <tr>
                        <th>Requested By</th>
                        <th>File Path</th>
                        <th>Department</th>
                        <th>Status</th>
                        <th>Requested At</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
            `;

            shredRequests.forEach(shredRequest => {
                tableContent += `
                    <tr>
                        <td>${shredRequest.requestby}</td>
                        <td>${shredRequest.filepath}</td>
                        <td>${shredRequest.department}</td>
                        <td>${shredRequest.requeststatus}</td>
                        <td>${shredRequest.requestat}</td>
                        <td>
                        <div class="dropdown">
                        <button class="btn btn-sm app-btn-dark dropdown-toggle" type="button" id="actionsDropdown${shredRequest.requestid}" data-bs-toggle="dropdown" aria-expanded="false">
                            Actions
                        </button>
                        <ul class="dropdown-menu" aria-labelledby="actionsDropdown${shredRequest.requestid}">
                            <li>
                                <button class="dropdown-item btn" data-btn="${shredRequest.filepath}" onclick="approveShredRequest(this)">Approve</button>
                            </li>
                            <li>
                                <button class="dropdown-item btn" data-btn="${shredRequest.filepath}" onclick="denyShredRequest(this)">Deny</button>
                            </li>
                        </ul>
                      </div>
                      
                        </td>
                    </tr>
                `;
            });

            tableContent += '</tbody>';
            shredRequestTable.innerHTML = tableContent;
        });
    } catch (error) {
        console.error(error);
    }
}


function approveShredRequest(approvebutton) {
    const filepath = approvebutton.getAttribute('data-btn');

    Swal.fire({
        title: 'Are you sure?',
        html: `You are about to approve the shred request for the file: <b>${filepath}</b>`,
        icon: 'warning',
        showCancelButton: true,
        confirmButtonColor: '#3085d6',
        cancelButtonColor: '#d33',
        confirmButtonText: 'Approve'
    });
}

function denyShredRequest(denyButton) {
    const filepath = denyButton.getAttribute('data-btn');

    Swal.fire({
        title: 'Are you sure?',
        html: `You are about to deny the shred request for the file: <b>${filepath}</b>`,
        icon: 'warning',
        showCancelButton: true,
        confirmButtonColor: '#3085d6',
        cancelButtonColor: '#d33',
        confirmButtonText: 'Deny'
    });
}