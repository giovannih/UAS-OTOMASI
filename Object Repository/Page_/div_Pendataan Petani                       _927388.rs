<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Pendataan Petani                       _927388</name>
   <tag></tag>
   <elementGuidId>2270dd0f-0b1e-43a5-b14f-880ef35d28c0</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.section</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='wrapper']/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>268ded35-798e-4545-bd24-509942f083f0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>section</value>
      <webElementGuid>a1374fa4-002f-4b71-abd8-6d9df917f416</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
                
                    
                                             
                        Pendataan Petani
                          Daftar Petani
                        
                        
                    

            
                
                    
                        
                            
                                Id User *
                                
                            
                            
                                Nama Petani *
                                
                            
                          
                                Tanggal Lahir *
                                
                            
                            
                                Nama Istri / Suami
                                
                            
                            
                                Alamat Petani *
                                
                            
                            
                                Provinsi *
                                
                                     Provinsi 
                                    ACEHBALIBANTENBENGKULUDI YOGYAKARTADKI JAKARTAGORONTALOJAMBIJAWA BARATJAWA TENGAHJAWA TIMURKALIMANTAN BARATKALIMANTAN SELATANKALIMANTAN TENGAHKALIMANTAN TIMURKALIMANTAN UTARAKEPULAUAN BANGKA BELITUNGKEPULAUAN RIAULAMPUNGMALUKUMALUKU UTARANUSA TENGGARA BARATNUSA TENGGARA TIMURPAPUAPAPUA BARATRIAUSULAWESI BARATSULAWESI SELATANSULAWESI TENGAHSULAWESI TENGGARASULAWESI UTARASUMATERA BARATSUMATERA SELATANSUMATERA UTARA 
                                

                            

                            
                                
                                   
                                        Kabupaten *
                                       
                                        KabupatenKABUPATEN BANTULKABUPATEN GUNUNG KIDULKABUPATEN KULON PROGOKABUPATEN SLEMANKOTA YOGYAKARTA
                                    
                                    
                                        Kecamatan *
                                        KecamatanMANTRIJERONKRATONMERGANGSANUMBULHARJOKOTAGEDEGONDOKUSUMANDANUREJANPAKUALAMANGONDOMANANNGAMPILANWIROBRAJANGEDONG TENGENJETISTEGALREJO
                                    
                                    
                                        Desa / Kelurahan *
                                        Desa / KelurahanGEDONGKIWOMANTRIJERONSURYODININGRATAN
                                    
                                                                                      
                            
                            
                                Nomor Telepon *
                                
                            
                            
                                Pendidikan Terakhir 
                                
                                    SD
                                    SMP
                                    SMA
                                    D1
                                    D2
                                    D3
                                    S1
                                    S2
                                    S3
                                
                            
                                
                                    Jumlah Lahan *
                                    
                                
                            
                                Jumlah Tanggungan *
                                
                            
                                
                                    Jumlah Tenaga Kerja Musiman *
                                    
                                
                            
                                Email *
                                
                            
                            
                                Agama *
                                
                                    Islam
                                    Hindu
                                    Katolik
                                    Kristen
                                    Budha
                                    Konghucu
                                
                            

                            
                                Deskripsi Keahlian *
                                
                            
                            
                                Status Petani *
                                
                                    
                                        Aktif
                                    
                                        Tidak Aktif
                                
                            
                            
                                Jenis Kelamin *
                                
                                    
                                        Laki-laki
                                    
                                        Perempuan
                                
                            
                           
                                Foto 
                                
                            
                            
                            
                                
                            
                           
                        
                        
                    
                
            
                    

 $(document).ready(function(){
    $(&quot;#provinsi&quot;).change(function(){
        showHint($(&quot;#provinsi&quot;).val());
    });
});                                             

function showHint(str) {
    if (str.length == 0) { 
        document.getElementById(&quot;Kab&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasil = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kab&quot;).empty();
                $(&quot;#Kab&quot;).append(&quot;&lt;option>Kabupaten&lt;/option>&quot;);
                for (var i = 0; i &lt; hasil.length - 1; ++i) {
                    $(&quot;#Kab&quot;).append(hasil[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkab.php?q=&quot; + str, true);
        xmlhttp.send();
    }
}



 $(document).ready(function(){
    $(&quot;#Kab&quot;).change(function(){
        showHintkec($(&quot;#Kab&quot;).val());
    });
});                                             

function showHintkec(strkec) {
    if (strkec.length == 0) { 
        document.getElementById(&quot;Kec&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkec = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kec&quot;).empty();
                $(&quot;#Kec&quot;).append(&quot;&lt;option>Kecamatan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkec.length - 1; ++i) {
                    $(&quot;#Kec&quot;).append(hasilkec[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkec.php?qkec=&quot; + strkec, true);
        xmlhttp.send();
    }
}




 $(document).ready(function(){
    $(&quot;#Kec&quot;).change(function(){
        showHintkel($(&quot;#Kec&quot;).val());
    });
});                                             

function showHintkel(strkel) {
    if (strkel.length == 0) { 
        document.getElementById(&quot;Desa&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkel = this.responseText.split(&quot;,&quot;);
                $(&quot;#Desa&quot;).empty();
                $(&quot;#Desa&quot;).append(&quot;&lt;option>Desa / Kelurahan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkel.length - 1; ++i) {
                    $(&quot;#Desa&quot;).append(hasilkel[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkel.php?qkel=&quot; + strkel, true);
        xmlhttp.send();
    }
}



</value>
      <webElementGuid>39b62a01-45ee-4d97-a6c7-f3aaaced3448</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;wrapper&quot;)/div[@class=&quot;section&quot;]</value>
      <webElementGuid>9b655bd1-4ee3-43dd-8b66-e2b1b068785d</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='wrapper']/div[2]</value>
      <webElementGuid>9dc4821b-e1b6-4e04-8b90-b578310b23dd</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Statistik Kelompok Tani'])[1]/following::div[1]</value>
      <webElementGuid>716a036a-3576-4cfd-9442-4168d78a805d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Detail Anggota Kelompok Tani'])[1]/following::div[1]</value>
      <webElementGuid>fbc21ca6-f9bd-4698-b4d1-f73829c9c901</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[2]</value>
      <webElementGuid>185794f6-64c6-4995-89c6-e80c7f02c399</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = '
            
                
                    
                                             
                        Pendataan Petani
                          Daftar Petani
                        
                        
                    

            
                
                    
                        
                            
                                Id User *
                                
                            
                            
                                Nama Petani *
                                
                            
                          
                                Tanggal Lahir *
                                
                            
                            
                                Nama Istri / Suami
                                
                            
                            
                                Alamat Petani *
                                
                            
                            
                                Provinsi *
                                
                                     Provinsi 
                                    ACEHBALIBANTENBENGKULUDI YOGYAKARTADKI JAKARTAGORONTALOJAMBIJAWA BARATJAWA TENGAHJAWA TIMURKALIMANTAN BARATKALIMANTAN SELATANKALIMANTAN TENGAHKALIMANTAN TIMURKALIMANTAN UTARAKEPULAUAN BANGKA BELITUNGKEPULAUAN RIAULAMPUNGMALUKUMALUKU UTARANUSA TENGGARA BARATNUSA TENGGARA TIMURPAPUAPAPUA BARATRIAUSULAWESI BARATSULAWESI SELATANSULAWESI TENGAHSULAWESI TENGGARASULAWESI UTARASUMATERA BARATSUMATERA SELATANSUMATERA UTARA 
                                

                            

                            
                                
                                   
                                        Kabupaten *
                                       
                                        KabupatenKABUPATEN BANTULKABUPATEN GUNUNG KIDULKABUPATEN KULON PROGOKABUPATEN SLEMANKOTA YOGYAKARTA
                                    
                                    
                                        Kecamatan *
                                        KecamatanMANTRIJERONKRATONMERGANGSANUMBULHARJOKOTAGEDEGONDOKUSUMANDANUREJANPAKUALAMANGONDOMANANNGAMPILANWIROBRAJANGEDONG TENGENJETISTEGALREJO
                                    
                                    
                                        Desa / Kelurahan *
                                        Desa / KelurahanGEDONGKIWOMANTRIJERONSURYODININGRATAN
                                    
                                                                                      
                            
                            
                                Nomor Telepon *
                                
                            
                            
                                Pendidikan Terakhir 
                                
                                    SD
                                    SMP
                                    SMA
                                    D1
                                    D2
                                    D3
                                    S1
                                    S2
                                    S3
                                
                            
                                
                                    Jumlah Lahan *
                                    
                                
                            
                                Jumlah Tanggungan *
                                
                            
                                
                                    Jumlah Tenaga Kerja Musiman *
                                    
                                
                            
                                Email *
                                
                            
                            
                                Agama *
                                
                                    Islam
                                    Hindu
                                    Katolik
                                    Kristen
                                    Budha
                                    Konghucu
                                
                            

                            
                                Deskripsi Keahlian *
                                
                            
                            
                                Status Petani *
                                
                                    
                                        Aktif
                                    
                                        Tidak Aktif
                                
                            
                            
                                Jenis Kelamin *
                                
                                    
                                        Laki-laki
                                    
                                        Perempuan
                                
                            
                           
                                Foto 
                                
                            
                            
                            
                                
                            
                           
                        
                        
                    
                
            
                    

 $(document).ready(function(){
    $(&quot;#provinsi&quot;).change(function(){
        showHint($(&quot;#provinsi&quot;).val());
    });
});                                             

function showHint(str) {
    if (str.length == 0) { 
        document.getElementById(&quot;Kab&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasil = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kab&quot;).empty();
                $(&quot;#Kab&quot;).append(&quot;&lt;option>Kabupaten&lt;/option>&quot;);
                for (var i = 0; i &lt; hasil.length - 1; ++i) {
                    $(&quot;#Kab&quot;).append(hasil[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkab.php?q=&quot; + str, true);
        xmlhttp.send();
    }
}



 $(document).ready(function(){
    $(&quot;#Kab&quot;).change(function(){
        showHintkec($(&quot;#Kab&quot;).val());
    });
});                                             

function showHintkec(strkec) {
    if (strkec.length == 0) { 
        document.getElementById(&quot;Kec&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkec = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kec&quot;).empty();
                $(&quot;#Kec&quot;).append(&quot;&lt;option>Kecamatan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkec.length - 1; ++i) {
                    $(&quot;#Kec&quot;).append(hasilkec[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkec.php?qkec=&quot; + strkec, true);
        xmlhttp.send();
    }
}




 $(document).ready(function(){
    $(&quot;#Kec&quot;).change(function(){
        showHintkel($(&quot;#Kec&quot;).val());
    });
});                                             

function showHintkel(strkel) {
    if (strkel.length == 0) { 
        document.getElementById(&quot;Desa&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkel = this.responseText.split(&quot;,&quot;);
                $(&quot;#Desa&quot;).empty();
                $(&quot;#Desa&quot;).append(&quot;&lt;option>Desa / Kelurahan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkel.length - 1; ++i) {
                    $(&quot;#Desa&quot;).append(hasilkel[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkel.php?qkel=&quot; + strkel, true);
        xmlhttp.send();
    }
}



' or . = '
            
                
                    
                                             
                        Pendataan Petani
                          Daftar Petani
                        
                        
                    

            
                
                    
                        
                            
                                Id User *
                                
                            
                            
                                Nama Petani *
                                
                            
                          
                                Tanggal Lahir *
                                
                            
                            
                                Nama Istri / Suami
                                
                            
                            
                                Alamat Petani *
                                
                            
                            
                                Provinsi *
                                
                                     Provinsi 
                                    ACEHBALIBANTENBENGKULUDI YOGYAKARTADKI JAKARTAGORONTALOJAMBIJAWA BARATJAWA TENGAHJAWA TIMURKALIMANTAN BARATKALIMANTAN SELATANKALIMANTAN TENGAHKALIMANTAN TIMURKALIMANTAN UTARAKEPULAUAN BANGKA BELITUNGKEPULAUAN RIAULAMPUNGMALUKUMALUKU UTARANUSA TENGGARA BARATNUSA TENGGARA TIMURPAPUAPAPUA BARATRIAUSULAWESI BARATSULAWESI SELATANSULAWESI TENGAHSULAWESI TENGGARASULAWESI UTARASUMATERA BARATSUMATERA SELATANSUMATERA UTARA 
                                

                            

                            
                                
                                   
                                        Kabupaten *
                                       
                                        KabupatenKABUPATEN BANTULKABUPATEN GUNUNG KIDULKABUPATEN KULON PROGOKABUPATEN SLEMANKOTA YOGYAKARTA
                                    
                                    
                                        Kecamatan *
                                        KecamatanMANTRIJERONKRATONMERGANGSANUMBULHARJOKOTAGEDEGONDOKUSUMANDANUREJANPAKUALAMANGONDOMANANNGAMPILANWIROBRAJANGEDONG TENGENJETISTEGALREJO
                                    
                                    
                                        Desa / Kelurahan *
                                        Desa / KelurahanGEDONGKIWOMANTRIJERONSURYODININGRATAN
                                    
                                                                                      
                            
                            
                                Nomor Telepon *
                                
                            
                            
                                Pendidikan Terakhir 
                                
                                    SD
                                    SMP
                                    SMA
                                    D1
                                    D2
                                    D3
                                    S1
                                    S2
                                    S3
                                
                            
                                
                                    Jumlah Lahan *
                                    
                                
                            
                                Jumlah Tanggungan *
                                
                            
                                
                                    Jumlah Tenaga Kerja Musiman *
                                    
                                
                            
                                Email *
                                
                            
                            
                                Agama *
                                
                                    Islam
                                    Hindu
                                    Katolik
                                    Kristen
                                    Budha
                                    Konghucu
                                
                            

                            
                                Deskripsi Keahlian *
                                
                            
                            
                                Status Petani *
                                
                                    
                                        Aktif
                                    
                                        Tidak Aktif
                                
                            
                            
                                Jenis Kelamin *
                                
                                    
                                        Laki-laki
                                    
                                        Perempuan
                                
                            
                           
                                Foto 
                                
                            
                            
                            
                                
                            
                           
                        
                        
                    
                
            
                    

 $(document).ready(function(){
    $(&quot;#provinsi&quot;).change(function(){
        showHint($(&quot;#provinsi&quot;).val());
    });
});                                             

function showHint(str) {
    if (str.length == 0) { 
        document.getElementById(&quot;Kab&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasil = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kab&quot;).empty();
                $(&quot;#Kab&quot;).append(&quot;&lt;option>Kabupaten&lt;/option>&quot;);
                for (var i = 0; i &lt; hasil.length - 1; ++i) {
                    $(&quot;#Kab&quot;).append(hasil[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkab.php?q=&quot; + str, true);
        xmlhttp.send();
    }
}



 $(document).ready(function(){
    $(&quot;#Kab&quot;).change(function(){
        showHintkec($(&quot;#Kab&quot;).val());
    });
});                                             

function showHintkec(strkec) {
    if (strkec.length == 0) { 
        document.getElementById(&quot;Kec&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkec = this.responseText.split(&quot;,&quot;);
                $(&quot;#Kec&quot;).empty();
                $(&quot;#Kec&quot;).append(&quot;&lt;option>Kecamatan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkec.length - 1; ++i) {
                    $(&quot;#Kec&quot;).append(hasilkec[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkec.php?qkec=&quot; + strkec, true);
        xmlhttp.send();
    }
}




 $(document).ready(function(){
    $(&quot;#Kec&quot;).change(function(){
        showHintkel($(&quot;#Kec&quot;).val());
    });
});                                             

function showHintkel(strkel) {
    if (strkel.length == 0) { 
        document.getElementById(&quot;Desa&quot;).innerHTML = &quot;&quot;;
        return;
    } else {
        var xmlhttp = new XMLHttpRequest();
        xmlhttp.onreadystatechange = function() {
            if (this.readyState == 4 &amp;&amp; this.status == 200) {
                //document.getElementById(&quot;txtHint&quot;).innerHTML = this.responseText;
                var hasilkel = this.responseText.split(&quot;,&quot;);
                $(&quot;#Desa&quot;).empty();
                $(&quot;#Desa&quot;).append(&quot;&lt;option>Desa / Kelurahan&lt;/option>&quot;);
                for (var i = 0; i &lt; hasilkel.length - 1; ++i) {
                    $(&quot;#Desa&quot;).append(hasilkel[i]);
                };
            }
        };
        xmlhttp.open(&quot;GET&quot;, &quot;gethintkel.php?qkel=&quot; + strkel, true);
        xmlhttp.send();
    }
}



')]</value>
      <webElementGuid>0b4c627a-b3d9-42d8-916a-2e4546b45070</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
