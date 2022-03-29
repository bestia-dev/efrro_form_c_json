# efrro_form_c_json

**Guest enters data for Form C and export it as JSON**  
***version: 1.0  date: 2019-12-29 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/efrro_form_c_json)***  

Try it:  
<https://bestia.dev/formc/>  
  
Quote from the Bureau of Immigration in India (form C is compulsory by law):  
"Any Hotel, Guest House, Dharmashala, Individual House, University, Hospital, Institute and Others etc. who provide accommodation to foreigners **must** submit the details of the residing foreigner in Form C to the Registration authorities within 24 hours of the arrival of the foreigner at their premises. This will help the registration authorities in locating and tracking the foreigners."  
<https://www.india.gov.in/online-form-c-bureau-immigration>  
  
The web page is supposed to be used on the guest's personal smartphone.  
This web page saves data only in the local storage of this device. No data is ever sent over the network.  
This web page works in all modern mobile browsers (chrome, firefox, safari). Avoid old or non updated browsers (for your own security and comfort).  
The result of the web page is a simple JSON text. It can be easily saved, copied, modified and sent.  
But all this actions are outside of this web page for security and privacy reasons.  
They are a personal choice of the user.  
The guest can use this same JSON text in every accommodation where the accommodation manager uses the chrome extension. If the manager doesn't know about the extension, the guest could instruct the accommodation manager and make life easier for both the guest and the manager.  

## chrome extension

This result JSON text is then sent to the accommodation manager. He uses the chrome extension to fill the online form C.  
<https://chrome.google.com/webstore/detail/fill-the-online-form-c-fr/echcbgpcbpnpjdnpckmomhdjgecifbaa>  
Source code and instructions:  
<https://github.com/bestia-dev/efrro_form_c_chrome_ext>  

## The long story

The accommodation managers in India must fill the government online Form C for every guest.  
This is boring, time consuming and error prone.  
The guests today must fill their data repeatedly on paper forms. Super annoying.  
In an ideal world the guest should have his digital data on his smartphone and just give this digital data to the accommodation manager. Just like an (digital) ID card.  
With this web page the guest writes his data only once and they are saved on his device locally. Than he uses this same JSON data for all the accommodations.  
JSON is just a text that can be easily copied. He can send it by email to the accommodation manager or by any other digital communication channel.  

## Opensource

The project is open source stored on github and the source code can be easily checked to prove there are not dangers like malware or viruses.  

## Support

Maybe the best way to support this web page is with a group with questions and answers  
<https://groups.google.com/forum/#!forum/bestiaformc>  

## Personalization

If you are happy with the web page please donate to <paypal.me/LucianoBestia>.
To show my gratitude I will personalize the web page with your accommodation's logo and data. Because there is a known email address, the guest then have the easy button "send by default email client". It is not possible to do this in the basic version. So it is more user-friendly to the guest. accommodation managers should contact me on <Luciano.Bestia2@gmail.com> to add their accommodation data, logo and email address.  
Look at the example:  
<https://bestia.dev/formc_beta/?id=sturmfrei_goa>  

