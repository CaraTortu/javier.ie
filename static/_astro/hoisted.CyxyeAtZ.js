import"./ViewTransitions.astro_astro_type_script_index_0_lang.BMqMAa2A.js";const r=document.getElementById("contact_form");r.addEventListener("submit",function(t){t.preventDefault();const n=document.getElementById("email"),o=document.getElementById("subject"),c=document.getElementById("message");fetch("/api/email",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({from:n.value,subject:o.value,body:c.value})}).then(e=>e.text()).then(e=>{alert(e)}).catch(e=>{console.error("Error:",e),alert("An error occurred while sending the message.")})});
