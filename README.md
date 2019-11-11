# efrro_form_c_json

User interface to enter data for e-frro form C and export it as json.  
It will be used mostly on guest's smartphones.  

## chrome extension

This json is then used with the chrome extension chext_fill_form_from_json to fill the e-frro form C.  
This is done by the hostel manager only.  
<https://chrome.google.com/webstore/detail/chextfillformfromjson/gdgkhgfgpfhnmiebaedlcaignonmjobe>

## The long story

The hostel managers in India must fill the government online Form C for every guest.  
This is boring, time consuming and error prone.  
The guests today must fill their data repeatedly on paper forms. Super annoying.  
In an ideal world the guest should have his digital data on his smartphone and just give this digital data to the hostel manager. Just like an (digital) ID card.  
With efrro_form_c_json the guest writes his data only once and they are saved on his device locally. Than he usee this same json data for all the hostels.  
Json is just a text that can be easily copied. He can send it by email to the hostel manager.  

## TODO

- use json to fill the same form data on another device  
- how to hard reload on android chrome? It retains the old version for too long.  
