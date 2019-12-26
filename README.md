# efrro_form_c_json

*Things are changing fast. This is the situation on 2019-11-11.*  
Efrro_form_c_json ia a webapp for the hostel guest to enter data for e-frro form C and export it as JSON.  
The webapp link is this:  
<https://bestia.dev/efrro_form_c_json/>  
  
Quote from the Bureau of Immigration in India (form C is compulsory by law):  
"Any Hotel, Guest House, Dharmashala, Individual House, University, Hospital, Institute and Others etc. who provide accommodation to foreigners **must** submit the details of the residing foreigner in Form C to the Registration authorities within 24 hours of the arrival of the foreigner at their premises. This will help the registration authorities in locating and tracking the foreigners."  
<https://www.india.gov.in/online-form-c-bureau-immigration>  
  
The webapp is supposed to be used on the guest's personal smartphone.  
This webapp saves data only in the local storage of this device. No data is ever sent over the network.  
This webapp works in all modern mobile browsers (chrome, firefox, safari). Avoid old or non updated browsers (for your own security and comfort).  
The result of the webapp is a simple JSON text. It can be easily saved, copied, modified and sent.  
But all this actions are outside of this webapp for security and privacy reasons.  
They are a personal choice of the user.  
The guest can use this same JSON text in every hostel where the hostel manager uses the chrome extension. If the manager doesn't know about the extension, the guest could instruct the hostel manager and make life easier for both the guest and the manager.  

## chrome extension

This result JSON text is then sent to the hostel manager. He uses the chrome extension to fill the online form C.  
<https://chrome.google.com/webstore/detail/fill-the-online-form-c-fr/echcbgpcbpnpjdnpckmomhdjgecifbaa>  
Source code and instructions:  
<https://github.com/LucianoBestia/efrro_form_c_chrome_ext>  

## The long story

The hostel managers in India must fill the government online Form C for every guest.  
This is boring, time consuming and error prone.  
The guests today must fill their data repeatedly on paper forms. Super annoying.  
In an ideal world the guest should have his digital data on his smartphone and just give this digital data to the hostel manager. Just like an (digital) ID card.  
With efrro_form_c_json the guest writes his data only once and they are saved on his device locally. Than he uses this same JSON data for all the hostels.  
JSON is just a text that can be easily copied. He can send it by email to the hostel manager or by any other digital communication channel.  

## Opensource

The project is open source stored on github and the source code can be easily checked to prove there are not dangers like malware or viruses.  

## Support
Maybe the best way to support this app is with a group with questions and answers  
<https://groups.google.com/forum/#!forum/bestiaformc>  

## Customization

It is possible to customize the webapp with the hostel's logo and data. Because there is a known email address, the guest then have the easy button "send by default email client". It is not possible to do this in the basic version. So it is more user-friendly to the guest. Hostel managers should contact me to add their hostel data, logo and email address for a small donation for cloud provider expenses.  
Look at the example:  
<https://bestia.dev/efrro_form_c_json_beta/?id=sturmfrei_goa>  

## TODO

- use JSON to fill the same form data on another device  
