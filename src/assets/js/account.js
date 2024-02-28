const invoke = window.__TAURI__.invoke
const notification = window.__TAURI__.notification

if (isAdminLoggedIn()) {
    const adminProfileForm = document.querySelector('#admin-profile-form');
    const adminId = localStorage.getItem('adminId');
    const adminUsername = localStorage.getItem('adminUsername');

    const adminUsernameInput = document.getElementById('admin-username-input');
    const adminNameInput = document.getElementById('admin-name-input')
    const adminEmailInput = document.getElementById('admin-email-input')
    const adminPhoneNoInput = document.getElementById('admin-phone-no-input')


    var adminName = null;
    var adminEmail = null;
    var adminPhoneNo = null;
    var adminDepartment = null;

    invoke('get_admin', {
        username: adminUsername,
    }).then(response => {
        let admin = response;
        adminName = admin.fullname;
        adminEmail = admin.email;
        adminPhoneNo = admin.phone;
        adminDepartment = admin.department;

        // set the values of the HTML elements
        document.getElementById('admin-username').innerHTML = `@${adminUsername}`;
        document.getElementById('admin-name').innerHTML = adminName;
        document.getElementById('admin-email').innerHTML = adminEmail;
        document.getElementById('admin-phone-no').innerHTML = adminPhoneNo;
        document.getElementById('admin-department').innerHTML = adminDepartment;

        // set the values for the input elements
        adminUsernameInput.value = adminUsername;
        adminNameInput.value = adminName;
        adminEmailInput.value = adminEmail;
        adminPhoneNoInput.value = adminPhoneNo;
    }
    ).catch(error => {
        notification.sendNotification({
            title: `Error!`,
            body: `${error}`, // ensure error is a string
        });

    })

}


if (isEmployeeLoggedIn()) {
    const employeeId = localStorage.getItem('employeeId');
    const employeeUsername = localStorage.getItem('employeeUsername');

    const employeeProfileForm = document.querySelector('#employee-profile-form');

    const employeeUsernameInput = document.getElementById('employee-username-input');
    const employeeNameInput = document.getElementById('employee-name-input')
    const employeeEmailInput = document.getElementById('employee-email-input')
    const employeePhoneNoInput = document.getElementById('employee-phone-no-input')


    var employeeName = null;
    var employeeEmail = null;
    var employeePhoneNo = null;
    var employeeDepartment = null;

    invoke('get_employee', {
        username: employeeUsername,
    }).then(response => {
        let employee = response;

        // set the values of the HTML elements
        document.getElementById('employee-username').innerHTML = `@${employeeUsername}`;
        document.getElementById('employee-name').innerHTML = employee.fullname;
        document.getElementById('employee-email').innerHTML = employee.email;
        document.getElementById('employee-phone-no').innerHTML = employee.phone;
        document.getElementById('employee-department').innerHTML = employee.department;

        // set the values for the input elements
        employeeUsernameInput.value = employeeUsername;
        employeeNameInput.value = employee.fullname;
        employeeEmailInput.value = employee.email;
        employeePhoneNoInput.value = employee.phone;
    }
    ).catch(error => {
        notification.sendNotification({
            title: `Error!`,
            body: `${error}`, // ensure error is a string
        });
    })

    // add update user fuctionality
    employeeProfileForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const employeeUsername = employeeUsernameInput.value;
        const employeeName = employeeNameInput.value;
        const employeeEmail = employeeEmailInput.value;
        const employeePhoneNo = employeePhoneNoInput.value;

        invoke('update_employee', {
            employeeid: employeeId,
            username: employeeUsername,
            fullname: employeeName,
            email: employeeEmail,
            phone: employeePhoneNo,
        }).then(response => {
            updateEmployeeSessionDetails(employeeId, employeeUsername, employeeName)
            Swal.fire({
                title: `Update successful!`,
                html: `Please wait while we apply the requested changes to your profile <b></b>`,
                allowOutsideClick: false,
                allowEscapeKey: false,
                timer: 3000,
                didOpen: () => {
                    Swal.showLoading();
                    Swal.getPopup().querySelector("b");
                },
            }).then((result) => {
                if (result.dismiss === Swal.DismissReason.timer) {
                    window.location.href = 'employee-account.html';
                }
            });
        }
        ).catch(error => {
            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `${error}`,
            })

        })
    });


    // add change password fuctionality
}