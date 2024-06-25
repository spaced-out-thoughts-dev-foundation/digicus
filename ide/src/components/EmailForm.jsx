import React from 'react'

import ".././styles/FileUpload.css";

function EmailForm() {
  return (
    <div
      dangerouslySetInnerHTML={{
        __html: `
          <!-- Begin Mailchimp Signup Form -->
          <div id="mc_embed_signup">
            <form action="https://dev.us14.list-manage.com/subscribe/post?u=96948c5e8fc6f534d39ebdb3f&amp;id=6080af26e0" method="post" id="mc-embedded-subscribe-form" name="mc-embedded-subscribe-form" class="validate" target="_blank" novalidate>
              <div id="mc_embed_signup_scroll">
                <h2>Subscribe to our mailing list</h2>
                <div class="mc-field-group">
                  <label for="mce-EMAIL">Email Address </label>
                  <input type="email" value="" name="EMAIL" class="required email" id="mce-EMAIL">
                </div>
                <div id="mce-responses" class="clear">
                  <div class="response" id="mce-error-response" style="display:none"></div>
                  <div class="response" id="mce-success-response" style="display:none"></div>
                </div>    
                <div style="position: absolute; left: -5000px;" aria-hidden="true">
                  <input type="text" name="b_96948c5e8fc6f534d39ebdb3f_6080af26e0" tabindex="-1" value="">
                </div>
                <div class="clear">
                  <input type="submit" value="Subscribe" name="subscribe" id="mc-embedded-subscribe" class="button">
                </div>
              </div>
            </form>
          </div>
          <!--End mc_embed_signup-->
        `
      }}
    />
  );
};


export default EmailForm;