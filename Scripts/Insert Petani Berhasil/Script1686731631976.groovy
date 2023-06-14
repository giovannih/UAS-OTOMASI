import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://genesis.dutatani.id/si_petani/login.php?pesan=Mohon%20Login')

WebUI.setText(findTestObject('Object Repository/Page_/input_ID User_id_user (2) (1) (1)'), 'tester_otomasi')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_/input_Password_password (2) (1) (1)'), 'hUKwJTbofgPU9eVlw/CnDQ==')

WebUI.sendKeys(findTestObject('Object Repository/Page_/input_Password_password (2) (1) (1)'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Object Repository/Page_/a_Pendataan Petani (2) (1)'))

WebUI.setText(findTestObject('Object Repository/Page_/input_Id User_id_user (3) (1) (1)'), '999666')

WebUI.setText(findTestObject('Object Repository/Page_/input_Nama Petani_nama_petani (1) (1)'), 'Jono Sijono')

WebUI.setText(findTestObject('Object Repository/Page_/input_Tanggal Lahir_tanggal_lahir (1)'), '02102002')

WebUI.setText(findTestObject('Object Repository/Page_/input_Nama Istri  Suami_nama_istri (1) (1)'), 'Udin siudin')

WebUI.setText(findTestObject('Object Repository/Page_/input_Alamat Petani_alamat_petani (1) (1)'), 'Yogyakarta')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_/select_Provinsi                            _919ea7 (1) (1)'), 
    'DI YOGYAKARTA', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_/select_KabupatenKABUPATEN BANTULKABUPATEN G_999db8 (1) (1)'), 
    'KOTA YOGYAKARTA', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_/select_KecamatanMANTRIJERONKRATONMERGANGSAN_999684 (1) (1)'), 
    'MANTRIJERON', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_/select_Desa  KelurahanGEDONGKIWOMANTRIJERON_b15df8 (1) (1)'), 
    'GEDONGKIWO', true)

WebUI.setText(findTestObject('Object Repository/Page_/input_Nomor Telepon_nomor_telpon (1) (1)'), '08111111111111')

WebUI.setText(findTestObject('Object Repository/Page_/input_Jumlah Lahan_jml_lahan (1) (1)'), '100')

WebUI.setText(findTestObject('Object Repository/Page_/input_Jumlah Tanggungan_jumlah_tanggungan (1) (1)'), '2')

WebUI.setText(findTestObject('Object Repository/Page_/input_Jumlah Tenaga Kerja Musiman_jml_tng_k_49b205 (1) (1)'), '10')

WebUI.setText(findTestObject('Object Repository/Page_/input_Email_email (1) (1)'), 'gregoriusgiovanni02@gmail.com')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_/select_Islam                               _f363b0 (1) (1)'), 
    'Katolik', true)

WebUI.setText(findTestObject('Object Repository/Page_/textarea_Deskripsi Keahlian_deskripsi_keahlian (1) (1)'), 'memahat')

WebUI.click(findTestObject('Object Repository/Page_/label_Aktif (1) (1)'))

WebUI.click(findTestObject('Object Repository/Page_/label_Laki-laki (1) (1)'))

WebUI.click(findTestObject('Object Repository/Page_/input_Foto_btn btn-primary btn-lg (1) (1)'))

WebUI.navigateToUrl('https://genesis.dutatani.id/si_petani/DataPetani.php?pesan=Data%20Petani%20baru%20tidak%20berhasil%20ditambahkan...&jenis=text-error')

WebUI.setText(findTestObject('Object Repository/Page_/input_Cetak Daftar_input_cari'), 'Jono Sijono')

WebUI.click(findTestObject('Object Repository/Page_/input_Cetak Daftar_cari'))

