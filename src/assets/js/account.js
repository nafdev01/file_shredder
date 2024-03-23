const { invoke } = window.__TAURI__;
const { notification } = window.__TAURI__;

const AccountSwal = Swal.mixin({
    showConfirmButton: false,
    didOpen: () => {
        Swal.showLoading();
        Swal.getPopup().querySelector("b");
    },
});

const passwordPattern = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[!@#$*]).{8,}$/;


if (isEmployeeLoggedIn()) {

    AccountSwal.fire({
        title: 'Loading profile ...',
        html: 'Please wait while we populate your profile with your details <b></b>',
    });

    const employeeId = localStorage.getItem('employeeId');
    const employeeUsername = localStorage.getItem('employeeUsername');

    const employeeProfileForm = document.querySelector('#employee-profile-form');
    const employeeChangePasswordForm = document.querySelector('#employee-change-password-form');

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

        AccountSwal.close();

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

        AccountSwal.close();

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

        AccountSwal.fire({
            title: 'Updating employee profile ...',
            html: 'Please wait while we update your profile <b></b>',
        });

        invoke('update_employee', {
            employeeid: parseInt(employeeId),
            username: employeeUsername,
            fullname: employeeName,
            email: employeeEmail,
            phone: employeePhoneNo,
        }).then(response => {

            AccountSwal.close();

            updateEmployeeSessionDetails(employeeId, employeeUsername, employeeName)
            window.location.href = 'employee-account.html';
        }
        ).catch(error => {

            AccountSwal.close();

            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `${error}`,
            })

        })
    });


    // add change password functionality
    employeeChangePasswordForm.addEventListener('submit', (e) => {
        e.preventDefault();

        const oldPassword = document.getElementById('employee-old-password-input').value;
        const newPassword = document.getElementById('employee-new-password-input').value;
        const confirmPassword = document.getElementById('employee-confirm-new-password-input').value;

        if (newPassword !== confirmPassword) {
            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `The new passwords do not match!`,
            })
            return;
        }

        if (!passwordPattern.test(newPassword)) {
            Swal.fire({
                title: 'Password is not strong enough!',
                html: `
                <ul>
                <li>Be at least 8 characters long</li>
                <li>Have at least one uppercase letter</li>
                <li>Have at least one lowercase letter</li>
                <li>Have at least one number</li>
                <li>Have at least one special character (like !, @, #, $,*)</li>
               </ul>`,
                icon: 'error',
                confirmButtonText: 'Ok'
            });
            return;
        }

        AccountSwal.fire({
            title: 'Changing password ...',
            html: 'Please wait while we change your password <b></b>',
        });

        invoke('change_employee_password', {
            employeeid: parseInt(employeeId),
            oldpassword: oldPassword,
            newpassword: newPassword,
        }).then(response => {

            AccountSwal.close();
            window.location.href = 'employee-account.html';

        }
        ).catch(error => {
            AccountSwal.close();

            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `${error}`,
            })

        })

    })
}

if (isAdminLoggedIn()) {
    const adminId = localStorage.getItem('adminId');
    const adminUsername = localStorage.getItem('adminUsername');

    const adminProfileForm = document.querySelector('#admin-profile-form');
    const adminChangePasswordForm = document.querySelector('#admin-change-password-form');

    const adminUsernameInput = document.getElementById('admin-username-input');
    const adminNameInput = document.getElementById('admin-name-input')
    const adminEmailInput = document.getElementById('admin-email-input')
    const adminPhoneNoInput = document.getElementById('admin-phone-no-input')


    var adminName = null;
    var adminEmail = null;
    var adminPhoneNo = null;
    var adminDepartment = null;

    AccountSwal.fire({
        title: 'Loading admin profile ...',
        html: 'Please wait while we populate your profile with your details <b></b>',
    });

    invoke('get_admin', {
        username: adminUsername,
    }).then(response => {

        AccountSwal.close();

        let admin = response;

        // set the values of the HTML elements
        document.getElementById('admin-username').innerHTML = `@${adminUsername}`;
        document.getElementById('admin-name').innerHTML = admin.fullname;
        document.getElementById('admin-email').innerHTML = admin.email;
        document.getElementById('admin-phone-no').innerHTML = admin.phone;
        document.getElementById('admin-department').innerHTML = admin.department;

        // set the values for the input elements
        adminUsernameInput.value = adminUsername;
        adminNameInput.value = admin.fullname;
        adminEmailInput.value = admin.email;
        adminPhoneNoInput.value = admin.phone;
    }
    ).catch(error => {

        AccountSwal.close();

        notification.sendNotification({
            title: `Error!`,
            body: `${error}`, // ensure error is a string
        });
    })

    // add update user fuctionality
    adminProfileForm.addEventListener('submit', (e) => {
        e.preventDefault();
        const adminUsername = adminUsernameInput.value;
        const adminName = adminNameInput.value;
        const adminEmail = adminEmailInput.value;

        AccountSwal.fire({
            title: 'Updating admin profile ...',
            html: 'Please wait while we update your profile <b></b>',
        });

        invoke('update_admin', {
            adminid: parseInt(adminId),
            username: adminUsername,
            fullname: adminName,
            email: adminEmail,
            phone: adminPhoneNo,
        }).then(response => {

            AccountSwal.close();

            updateAdminSessionDetails(adminId, adminUsername, adminName)
            window.location.href = 'admin-account.html';

        }
        ).catch(error => {

            AccountSwal.close();

            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `${error}`,
            })

        })
    });


    // add change password functionality
    adminChangePasswordForm.addEventListener('submit', (e) => {
        e.preventDefault();

        const oldPassword = document.getElementById('admin-old-password-input').value;
        const newPassword = document.getElementById('admin-new-password-input').value;
        const confirmPassword = document.getElementById('admin-confirm-new-password-input').value;

        if (newPassword !== confirmPassword) {
            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `The new passwords do not match!`,
            })
            return;
        }

        if (!passwordPattern.test(newPassword)) {
            Swal.fire({
                title: 'Password is not strong enough!',
                html: `
                <ul>
                <li>Be at least 8 characters long</li>
                <li>Have at least one uppercase letter</li>
                <li>Have at least one lowercase letter</li>
                <li>Have at least one number</li>
                <li>Have at least one special character (like !, @, #, $,*)</li>
               </ul>`,
                icon: 'error',
                confirmButtonText: 'Ok'
            });
            return;
        }

        AccountSwal.fire({
            title: 'Changing password ...',
            html: 'Please wait while we change your password <b></b>',
        });

        invoke('change_admin_password', {
            adminid: parseInt(adminId),
            oldpassword: oldPassword,
            newpassword: newPassword,
        }).then(response => {

            AccountSwal.close();
            window.location.href = 'admin-account.html';

        }
        ).catch(error => {

            AccountSwal.close();

            Swal.fire({
                icon: 'error',
                title: 'Oops!',
                text: `${error}`,
            })

        })

    })
}